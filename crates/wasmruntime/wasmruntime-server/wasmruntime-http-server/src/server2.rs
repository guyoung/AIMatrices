use std::collections::HashMap;
use std::io::IsTerminal;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{anyhow, bail, Context};
use http::uri::{Authority, Scheme};
use http::Uri;
use http_body_util::BodyExt;
use tokio::net::TcpListener;
use serde::{ Deserialize, Serialize };

use axum::body::Bytes;
use axum::extract::ConnectInfo;
use axum::extract::DefaultBodyLimit;
use axum::extract::Request;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Response, IntoResponse};
use axum::routing::any;
use axum::Router;

use axum_auth::AuthBasic;

use spin_app::APP_NAME_KEY;
use spin_factor_outbound_networking::is_service_chaining_host;
use spin_http::app_info::AppInfo;
use spin_http::config::{HttpExecutorType, HttpTriggerConfig};
use spin_http::routes::RouteMatch;
use spin_http::trigger::HandlerType;
use spin_factor_outbound_http::{OutboundHttpFactor, SelfRequestOrigin};

use wasmruntime_http_trigger::wagi::WagiHttpExecutor;
use wasmruntime_http_trigger::wasi::WasiHttpExecutor;
use wasmruntime_http_trigger::HttpExecutor;

use crate::{HttpTriggerApp, NotFoundRouteKind};

#[derive(Clone)]
struct AppState {
    listen_addr: SocketAddr,
    trigger_app: Arc<crate::HttpTriggerApp>,
    router: Arc<spin_http::routes::Router>,
    component_trigger_configs: Arc<HashMap<String, HttpTriggerConfig>>,
    component_handler_types: Arc<HashMap<String, HandlerType>>,
    user: Option<String>,
    pass: Option<String>
}

pub struct HttpServer {
    /// The address the server is listening on.
    listen_addr: SocketAddr,
    /// The app being triggered.
    trigger_app: Arc<crate::HttpTriggerApp>,
    /// Request router.
    router: spin_http::routes::Router,
    // Component ID -> component trigger config
    component_trigger_configs: HashMap<String, HttpTriggerConfig>,
    // Component ID -> handler type
    component_handler_types: HashMap<String, HandlerType>,
    user: Option<String>,
    pass: Option<String>
}

impl HttpServer {
    pub fn new(
        listen_addr: SocketAddr,
        trigger_app: Arc<crate::HttpTriggerApp>,
        user: Option<String>,
        pass: Option<String>
    ) -> anyhow::Result<Self> {
        // This needs to be a vec before building the router to handle duplicate routes
        let component_trigger_configs = Vec::from_iter(
            trigger_app
                .app()
                .trigger_configs::<HttpTriggerConfig>("http")?
                .into_iter()
                .map(|(_, config)| (config.component.clone(), config)),
        );

        // Build router
        let component_routes = component_trigger_configs
            .iter()
            .map(|(component_id, config)| (component_id.as_str(), &config.route));
        let (router, duplicate_routes) = spin_http::routes::Router::build("/", component_routes)?;
        if !duplicate_routes.is_empty() {
            tracing::error!(
                "The following component routes are duplicates and will never be used:"
            );
            for dup in &duplicate_routes {
                tracing::error!(
                    "  {}: {} (duplicate of {})",
                    dup.replaced_id,
                    dup.route(),
                    dup.effective_id,
                );
            }
        }
        // println!(
        //    "Constructed router: {:?}",
        //    router.routes().collect::<Vec<_>>()
        //);

        // Now that router is built we can merge duplicate routes by component
        let component_trigger_configs = HashMap::from_iter(component_trigger_configs);

        let component_handler_types = component_trigger_configs
            .iter()
            .map(|(component_id, trigger_config)| {
                let handler_type = match &trigger_config.executor {
                    None | Some(HttpExecutorType::Http) => {
                        let component = trigger_app.get_component(component_id)?;
                        HandlerType::from_component(trigger_app.engine().as_ref(), component)?
                    }
                    Some(HttpExecutorType::Wagi(wagi_config)) => {
                        anyhow::ensure!(
                            wagi_config.entrypoint == "_start",
                            "Wagi component '{component_id}' cannot use deprecated 'entrypoint' field"
                        );
                        HandlerType::Wagi
                    }
                };
                Ok((component_id.clone(), handler_type))
            })
            .collect::<anyhow::Result<_>>()?;

        Ok(Self {
            listen_addr,
            trigger_app,
            router,
            component_trigger_configs,
            component_handler_types,
            user,
            pass
        })
    }

    pub async fn serve(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.listen_addr).await.with_context(|| {
            format!(
                "Unable to listen on {listen_addr}",
                listen_addr = self.listen_addr
            )
        })?;

        print_startup_msgs("http", &listener, &self.router)?;

        let app_state = AppState {
            listen_addr: self.listen_addr.clone(),
            trigger_app: self.trigger_app.clone(),
            router: Arc::new(self.router.clone()),
            component_trigger_configs: Arc::new(self.component_trigger_configs.clone()),
            component_handler_types: Arc::new(self.component_handler_types.clone()),
            user: self.user.clone(),
            pass: self.pass.clone()
        };

        let app = Router::new();

        // let web_dir = PathBuf::from("./app_data/web");
        // let static_files_service = ServeDir::new(web_dir).append_index_html_on_directories(true);
        // let app = app.fallback_service(static_files_service);

        let app = config_route(app);

        let app = app.with_state(app_state);

        axum::serve(
            listener,
            app.layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .map_err(|e| anyhow!(format!("{:?}", e)))?;

        Ok(())
    }
}

fn print_startup_msgs(
    scheme: &str,
    listener: &TcpListener,
    router: &spin_http::routes::Router,
) -> anyhow::Result<()> {
    let local_addr = listener.local_addr()?;
    let base_url = format!("{scheme}://{local_addr:?}");
    println!("Serving {base_url}");

    println!("Available Routes:");
    for (route, component_id) in router.routes() {
        println!("  {}: {}{}", component_id, base_url, route);
    }
    Ok(())
}

fn config_route(routes: Router<AppState>) -> Router<AppState> {
    //let mut routes = routes.clone().route("/healthz", get(handle_healthz));

    //let routes = routes.clone().route("/service/{*path}", any(handle));

    let routes = routes.clone().route("/", any(handle));
    let routes = routes.clone().route("/{*path}", any(handle));

    return routes;
}

/*
async fn handle_healthz() -> impl IntoResponse {
    Html("<h1>Hello, World!</h1>")
}
*/

async fn handle(
    State(state): State<AppState>,
    auth: BasicAuthFromRequest,
    ConnectInfo(client_addr): ConnectInfo<SocketAddr>,
    request: Request,
) -> impl IntoResponse {
    //println!("http server handle");

    if state.user.is_some() {
        if auth.user != state.user || (state.pass.is_some() && auth.pass != state.pass) {
            return unauth();
        }
    }


    let mut request = request;

    let _method = request.method().clone();
    let uri = request.uri().clone();
    let scheme = uri.scheme().unwrap_or(&Scheme::HTTP);

    // println!("Received request: {} {}", method, uri);

    // println!("client_addr: {}", client_addr);

    strip_forbidden_headers(&mut request);

    let path = request.uri().path().to_string();

    //println!("Processing request on path '{path}'");

   const WELL_KNOWN_PREFIX: &str = "/.well-known/server/";

    if let Some(well_known) = path.strip_prefix(WELL_KNOWN_PREFIX) {
        return match well_known {
            "health" => http::Response::new(spin_http::body::full(Bytes::from_static(b"OK"))),
            "info" => app_info(state.trigger_app.as_ref()),
            _ => not_found(NotFoundRouteKind::WellKnown),
        };
    }

    match state.router.route(&path) {
        Ok(route_match) => {
            let res = handle_trigger_route(
                &state.trigger_app,
                &state.component_trigger_configs,
                &state.component_handler_types,
                request,
                &route_match,
                &state.listen_addr,
                scheme,
                client_addr,
            )
            .await;

            match res {
                Ok(res) => res,
                Err(_err) => {
                    //println!("Error processing request: {err:?}");
                    internal_error(None)
                }
            }
        }
        Err(_) => not_found(NotFoundRouteKind::Normal(path.to_string())),
    }
}

fn unauth() ->  http::Response<spin_http::Body> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header("WWW-Authenticate", "Basic")
        .body(spin_http::body::full("Unauthorized".into()))
        .unwrap()
        .into()
}



/// Handles a successful route match.
async fn handle_trigger_route(
    trigger_app: &HttpTriggerApp,
    component_trigger_configs: &HashMap<String, HttpTriggerConfig>,
    component_handler_types: &HashMap<String, HandlerType>,
    req: Request,
    route_match: &RouteMatch,
    listen_addr: &SocketAddr,
    server_scheme: &Scheme,
    client_addr: SocketAddr,
) -> anyhow::Result<http::Response<spin_http::Body>> {
    //println!("http server handle trigger route");

    let (parts, body) = req.into_parts();

    let bytes = body.collect().await?.to_bytes();

    let mut req = Request::from_parts(parts, spin_http::body::full(bytes));

    set_req_uri(&mut req, server_scheme.clone())?;

    let _app_id = trigger_app
        .app()
        .get_metadata(APP_NAME_KEY)?
        .unwrap_or_else(|| "<unnamed>".into());

    let component_id = route_match.component_id();

    // println!(
    //    "http server handle trigger route - component_id: {}",
    //    component_id
    // );

    let mut instance_builder = trigger_app.prepare(component_id)?;

    // Set up outbound HTTP request origin and service chaining
    // The outbound HTTP factor is required since both inbound and outbound wasi HTTP
    // implementations assume they use the same underlying wasmtime resource storage.
    // Eventually, we may be able to factor this out to a separate factor.
    let outbound_http = instance_builder
        .factor_builder::<OutboundHttpFactor>()
        .context(
            "The wasi HTTP trigger was configured without the required wasi outbound http support",
        )?;
    let origin = SelfRequestOrigin::create(server_scheme.clone(), &listen_addr.to_string())?;
    outbound_http.set_self_request_origin(origin);

    //outbound_http.set_request_interceptor(OutboundHttpInterceptor::new(self.clone()))?;


    // Prepare HTTP executor
    let trigger_config = component_trigger_configs.get(component_id).unwrap();
    let handler_type = component_handler_types.get(component_id).unwrap();
    let executor = trigger_config
        .executor
        .as_ref()
        .unwrap_or(&HttpExecutorType::Http);

    let res = match executor {
        HttpExecutorType::Http => match handler_type {
            HandlerType::Wasi0_2 => {
                WasiHttpExecutor {
                    handler_type: *handler_type,
                }
                .execute(instance_builder, &route_match, req, client_addr)
                .await
            }
            HandlerType::Wagi => unreachable!(),
        },
        HttpExecutorType::Wagi(wagi_config) => {
            let executor = WagiHttpExecutor {
                wagi_config: wagi_config.clone(),
            };
            executor
                .execute(instance_builder, &route_match, req, client_addr)
                .await
        }
    };

    res
}

fn strip_forbidden_headers(req: &mut Request) {
    let headers = req.headers_mut();
    if let Some(host_header) = headers.get("Host") {
        if let Ok(host) = host_header.to_str() {
            if is_service_chaining_host(host) {
                headers.remove("Host");
            }
        }
    }
}

fn app_info(trigger_app: &HttpTriggerApp) -> http::Response<spin_http::Body> {
    let info = AppInfo::new(trigger_app.app());
    let body = serde_json::to_vec_pretty(&info).unwrap_or_default();

    http::Response::builder()
        .header("content-type", "application/json")
        .body(spin_http::body::full(body.into()))
        .unwrap_or_default()
}

/// Creates an HTTP 500 response.
fn internal_error(body: Option<&str>) -> http::Response<spin_http::Body> {
    let body = match body {
        Some(body) => spin_http::body::full(Bytes::copy_from_slice(body.as_bytes())),
        None => spin_http::body::empty(),
    };

    http::Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(body)
        .unwrap_or_default()
}

/// Creates an HTTP 404 response.
fn not_found(kind: NotFoundRouteKind) -> http::Response<spin_http::Body> {
    use std::sync::atomic::{AtomicBool, Ordering};
    static SHOWN_GENERIC_404_WARNING: AtomicBool = AtomicBool::new(false);
    if let NotFoundRouteKind::Normal(route) = kind {
        if !SHOWN_GENERIC_404_WARNING.fetch_or(true, Ordering::Relaxed)
            && std::io::stderr().is_terminal()
        {
            println!("Request to {route} matched no pattern, and received a generic 404 response. To serve a more informative 404 page, add a catch-all (/...) route.");
        }
    }
    http::Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(spin_http::body::empty())
        .unwrap_or_default()
}

/// The incoming request's scheme and authority
///
/// The incoming request's URI is relative to the server, so we need to set the scheme and authority.
/// Either the `Host` header or the request's URI's authority is used as the source of truth for the authority.
/// This function will error if the authority cannot be unambiguously determined.
fn set_req_uri(req: &mut Request<spin_http::Body>, scheme: Scheme) -> anyhow::Result<()> {
    let uri = req.uri().clone();
    let mut parts = uri.into_parts();
    let headers = req.headers();
    let header_authority = headers
        .get(http::header::HOST)
        .map(|h| -> anyhow::Result<Authority> {
            let host_header = h.to_str().context("'Host' header is not valid UTF-8")?;
            host_header
                .parse()
                .context("'Host' header contains an invalid authority")
        })
        .transpose()?;
    let uri_authority = parts.authority;

    // Get authority either from request URI or from 'Host' header
    let authority = match (header_authority, uri_authority) {
        (None, None) => bail!("no 'Host' header present in request"),
        (None, Some(a)) => a,
        (Some(a), None) => a,
        (Some(a1), Some(a2)) => {
            // Ensure that if `req.authority` is set, it matches what was in the `Host` header
            // https://github.com/hyperium/hyper/issues/1612
            if a1 != a2 {
                return Err(anyhow::anyhow!(
                    "authority in 'Host' header does not match authority in URI"
                ));
            }
            a1
        }
    };
    parts.scheme = Some(scheme);
    parts.authority = Some(authority);
    *req.uri_mut() = Uri::from_parts(parts).unwrap();
    Ok(())
}


#[derive(Deserialize,Serialize, Debug)]
struct BasicAuthFromRequest {
    pub user: Option<String>,
    pub pass: Option<String>,
}


impl<S> axum::extract::FromRequestParts<S> for BasicAuthFromRequest
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    // FIXME: We also have a configuration flag do run without authentication
    // This must be handled here too ... otherwise we get an Auth header missing error.
    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {

        let auth = AuthBasic::from_request_parts(parts, state).await;

        if let Ok(auth) = auth {
            let user = Some(auth.0.0);
            let pass = auth.0.1;

            Ok(Self {
                user,
                pass
            })
        } else {
            Ok(Self {
                user: None,
                pass: None
            })
        }


    }
}

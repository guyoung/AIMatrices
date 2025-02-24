//! Route matching for the HTTP trigger.

#![deny(missing_docs)]

use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use std::{collections::HashMap, fmt};

use crate::config::HttpTriggerRouteConfig;

/// Router for the HTTP trigger.
#[derive(Clone, Debug)]
pub struct Router {
    /// Resolves paths to routing information - specifically component IDs
    /// but also recording about the original route.
    router: std::sync::Arc<routefinder::Router<RouteHandler>>,
}

/// What a route maps to
#[derive(Clone, Debug)]
struct RouteHandler {
    /// The component ID that the route maps to.
    component_id: String,
    /// The route, including any application base.
    based_route: String,
    /// The route, not including any application base.
    raw_route: String,
    /// The route, including any application base and capturing information about whether it has a trailing wildcard.
    /// (This avoids re-parsing the route string.)
    parsed_based_route: ParsedRoute,
}

/// A detected duplicate route.
#[derive(Debug)] // Needed to call `expect_err` on `Router::build`
pub struct DuplicateRoute {
    /// The duplicated route pattern.
    route: String,
    /// The raw route that was duplicated.
    pub replaced_id: String,
    /// The component ID corresponding to the duplicated route.
    pub effective_id: String,
}

impl Router {
    /// Builds a router based on application configuration.
    pub fn build<'a>(
        base: &str,
        component_routes: impl IntoIterator<Item = (&'a str, &'a HttpTriggerRouteConfig)>,
    ) -> Result<(Self, Vec<DuplicateRoute>)> {
        // Some information we need to carry between stages of the builder.
        struct RoutingEntry<'a> {
            based_route: String,
            raw_route: &'a str,
            component_id: &'a str,
        }

        let mut routes = IndexMap::new();
        let mut duplicates = vec![];

        // Filter out private endpoints and capture the routes.
        let routes_iter = component_routes
            .into_iter()
            .filter_map(|(component_id, route)| {
                match route {
                    HttpTriggerRouteConfig::Route(raw_route) => {
                        let based_route = sanitize_with_base(base, raw_route);
                        Some(Ok(RoutingEntry { based_route, raw_route, component_id }))
                    }
                    HttpTriggerRouteConfig::Private(endpoint) => if endpoint.private {
                        None
                    } else {
                        Some(Err(anyhow!("route must be a string pattern or '{{ private = true }}': component '{component_id}' has {{ private = false }}")))
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?;

        // Remove duplicates.
        for re in routes_iter {
            let effective_id = re.component_id.to_string();
            let replaced = routes.insert(re.raw_route, re);
            if let Some(replaced) = replaced {
                duplicates.push(DuplicateRoute {
                    route: replaced.based_route,
                    replaced_id: replaced.component_id.to_string(),
                    effective_id,
                });
            }
        }

        // Build a `routefinder` from the remaining routes.

        let mut rf = routefinder::Router::new();

        for re in routes.into_values() {
            let (rfroute, parsed) = Self::parse_route(&re.based_route).map_err(|e| {
                anyhow!(
                    "Error parsing route {} associated with component {}: {e}",
                    re.based_route,
                    re.component_id
                )
            })?;

            let handler = RouteHandler {
                component_id: re.component_id.to_string(),
                based_route: re.based_route,
                raw_route: re.raw_route.to_string(),
                parsed_based_route: parsed,
            };

            rf.add(rfroute, handler).map_err(|e| anyhow!("{e}"))?;
        }

        let router = Self {
            router: std::sync::Arc::new(rf),
        };

        Ok((router, duplicates))
    }

    fn parse_route(based_route: &str) -> Result<(routefinder::RouteSpec, ParsedRoute), String> {
        if let Some(wild_suffixed) = based_route.strip_suffix("/...") {
            let rs = format!("{wild_suffixed}/*").try_into()?;
            let parsed = ParsedRoute::trailing_wildcard(wild_suffixed);
            Ok((rs, parsed))
        } else if let Some(wild_suffixed) = based_route.strip_suffix("/*") {
            let rs = based_route.try_into()?;
            let parsed = ParsedRoute::trailing_wildcard(wild_suffixed);
            Ok((rs, parsed))
        } else {
            let rs = based_route.try_into()?;
            let parsed = ParsedRoute::exact(based_route);
            Ok((rs, parsed))
        }
    }

    /// Returns the constructed routes.
    pub fn routes(&self) -> impl Iterator<Item = (&(impl fmt::Display + fmt::Debug), &String)> {
        self.router
            .iter()
            .map(|(_spec, handler)| (&handler.parsed_based_route, &handler.component_id))
    }

    /// This returns the component ID that should handle the given path, or an error
    /// if no component matches.
    ///
    /// If multiple components could potentially handle the same request based on their
    /// defined routes, components with matching exact routes take precedence followed
    /// by matching wildcard patterns with the longest matching prefix.
    pub fn route(&self, p: &str) -> Result<RouteMatch> {
        let best_match = self
            .router
            .best_match(p)
            .ok_or_else(|| anyhow!("Cannot match route for path {p}"))?;

        let route_handler = best_match.handler().clone();
        let named_wildcards = best_match
            .captures()
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect();
        let trailing_wildcard = best_match.captures().wildcard().map(|s|
            // Backward compatibility considerations - Spin has traditionally
            // captured trailing slashes, but routefinder does not.
            match (s.is_empty(), p.ends_with('/')) {
                // route: /foo/..., path: /foo
                (true, false) => s.to_owned(),
                // route: /foo/..., path: /foo/
                (true, true) => "/".to_owned(),
                // route: /foo/..., path: /foo/bar
                (false, false) => format!("/{s}"),
                // route: /foo/..., path: /foo/bar/
                (false, true) => format!("/{s}/"),
            }
        );

        Ok(RouteMatch {
            route_handler,
            named_wildcards,
            trailing_wildcard,
        })
    }
}

impl DuplicateRoute {
    /// The duplicated route pattern.
    pub fn route(&self) -> &str {
        if self.route.is_empty() {
            "/"
        } else {
            &self.route
        }
    }
}

#[derive(Clone, Debug)]
enum ParsedRoute {
    Exact(String),
    TrailingWildcard(String),
}

impl ParsedRoute {
    fn exact(route: impl Into<String>) -> Self {
        Self::Exact(route.into())
    }

    fn trailing_wildcard(route: impl Into<String>) -> Self {
        Self::TrailingWildcard(route.into())
    }
}

impl fmt::Display for ParsedRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ParsedRoute::Exact(path) => write!(f, "{}", path),
            ParsedRoute::TrailingWildcard(pattern) => write!(f, "{} (wildcard)", pattern),
        }
    }
}

/// A routing match for a URL.
pub struct RouteMatch {
    route_handler: RouteHandler,
    named_wildcards: HashMap<String, String>,
    trailing_wildcard: Option<String>,
}

impl RouteMatch {
    /// A synthetic match as if the given path was matched against the wildcard route.
    /// Used in service chaining.
    pub fn synthetic(component_id: &str, path: &str) -> Self {
        Self {
            route_handler: RouteHandler {
                component_id: component_id.to_string(),
                based_route: "/...".to_string(),
                raw_route: "/...".to_string(),
                parsed_based_route: ParsedRoute::TrailingWildcard(String::new()),
            },
            named_wildcards: Default::default(),
            trailing_wildcard: Some(path.to_string()),
        }
    }

    /// The matched component.
    pub fn component_id(&self) -> &str {
        &self.route_handler.component_id
    }

    /// The matched route, as originally written in the manifest, combined with the base.
    pub fn based_route(&self) -> &str {
        &self.route_handler.based_route
    }

    /// The matched route, excluding any trailing wildcard, combined with the base.
    pub fn based_route_or_prefix(&self) -> String {
        self.route_handler
            .based_route
            .strip_suffix("/...")
            .unwrap_or(&self.route_handler.based_route)
            .to_string()
    }

    /// The matched route, as originally written in the manifest.
    pub fn raw_route(&self) -> &str {
        &self.route_handler.raw_route
    }

    /// The matched route, excluding any trailing wildcard.
    pub fn raw_route_or_prefix(&self) -> String {
        self.route_handler
            .raw_route
            .strip_suffix("/...")
            .unwrap_or(&self.route_handler.raw_route)
            .to_string()
    }

    /// The named wildcards captured from the path, if any
    pub fn named_wildcards(&self) -> &HashMap<String, String> {
        &self.named_wildcards
    }

    /// The trailing wildcard part of the path, if any
    pub fn trailing_wildcard(&self) -> String {
        self.trailing_wildcard.clone().unwrap_or_default()
    }
}

/// Sanitizes the base and path and return a formed path.
fn sanitize_with_base<S: Into<String>>(base: S, path: S) -> String {
    let path = absolutize(path);

    format!("{}{}", sanitize(base.into()), sanitize(path))
}

fn absolutize<S: Into<String>>(s: S) -> String {
    let s = s.into();
    if s.starts_with('/') {
        s
    } else {
        format!("/{s}")
    }
}

/// Strips the trailing slash from a string.
fn sanitize<S: Into<String>>(s: S) -> String {
    let s = s.into();
    // TODO
    // This only strips a single trailing slash.
    // Should we attempt to strip all trailing slashes?
    match s.strip_suffix('/') {
        Some(s) => s.into(),
        None => s,
    }
}

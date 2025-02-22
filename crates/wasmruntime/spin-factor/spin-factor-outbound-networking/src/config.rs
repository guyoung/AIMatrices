use std::ops::Range;

use anyhow::{bail, ensure, Context};
use spin_app::{App, AppComponent};
use spin_locked_app::MetadataKey;

const ALLOWED_HOSTS_KEY: MetadataKey<Vec<String>> = MetadataKey::new("allowed_outbound_hosts");
const ALLOWED_HTTP_KEY: MetadataKey<Vec<String>> = MetadataKey::new("allowed_http_hosts");

pub const SERVICE_CHAINING_DOMAIN: &str = "spin.internal";
pub const SERVICE_CHAINING_DOMAIN_SUFFIX: &str = ".spin.internal";

/// Get the raw values of the `allowed_outbound_hosts` locked app metadata key.
///
/// This has support for converting the old `allowed_http_hosts` key to the new `allowed_outbound_hosts` key.
pub fn allowed_outbound_hosts(component: &AppComponent) -> anyhow::Result<Vec<String>> {
    let mut allowed_hosts = component
        .get_metadata(ALLOWED_HOSTS_KEY)
        .with_context(|| {
            format!(
                "locked app metadata was malformed for key {}",
                ALLOWED_HOSTS_KEY.as_ref()
            )
        })?
        .unwrap_or_default();

    let allowed_http = component
        .get_metadata(ALLOWED_HTTP_KEY)
        .map(|h| h.unwrap_or_default())
        .unwrap_or_default();

    let converted = convert_allowed_http_to_allowed_hosts(&allowed_http, false).unwrap_or_default();
    allowed_hosts.extend(converted);
    Ok(allowed_hosts)
}

fn convert_allowed_http_to_allowed_hosts(
    allowed_http_hosts: &[impl AsRef<str>],
    allow_database_access: bool,
) -> anyhow::Result<Vec<String>> {
    let http_hosts = crate::allowed_http_hosts::parse_allowed_http_hosts(allowed_http_hosts)?;

    let mut outbound_hosts = if allow_database_access {
        vec![
            "redis://*:*".into(),
            "mysql://*:*".into(),
            "postgres://*:*".into(),
        ]
    } else {
        Vec::new()
    };
    match http_hosts {
        crate::allowed_http_hosts::AllowedHttpHosts::AllowAll => outbound_hosts.extend([
            "http://*:*".into(),
            "https://*:*".into(),
            "http://self".into(),
        ]),
        crate::allowed_http_hosts::AllowedHttpHosts::AllowSpecific(specific) => outbound_hosts
            .extend(specific.into_iter().flat_map(|s| {
                if s.domain == "self" {
                    vec!["http://self".into()]
                } else {
                    let port = s.port.map(|p| format!(":{p}")).unwrap_or_default();
                    vec![
                        format!("http://{}{}", s.domain, port),
                        format!("https://{}{}", s.domain, port),
                    ]
                }
            })),
    };
    Ok(outbound_hosts)
}

/// Validates that all service chaining of an app will be satisfied by the
/// supplied subset of components.
///
/// This does a best effort look up of components that are
/// allowed to be accessed through service chaining and will error early if a
/// component is configured to to chain to another component that is not
/// retained. All wildcard service chaining is disallowed and all templated URLs
/// are ignored.
pub fn validate_service_chaining_for_components(
    app: &App,
    retained_components: &[&str],
) -> anyhow::Result<()> {
    app
        .triggers().try_for_each(|t| {
            let Ok(component) = t.component() else  { return Ok(()) };
            if retained_components.contains(&component.id()) {
            let allowed_hosts = allowed_outbound_hosts(&component).context("failed to get allowed hosts")?;
            for host in allowed_hosts {
                // Templated URLs are not yet resolved at this point, so ignore unresolvable URIs
                if let Ok(uri) = host.parse::<http::Uri>() {
                    if let Some(chaining_target) = parse_service_chaining_target(&uri) {
                        if !retained_components.contains(&chaining_target.as_ref()) {
                            if chaining_target == "*" {
                                return  Err(anyhow::anyhow!("Selected component '{}' cannot use wildcard service chaining: allowed_outbound_hosts = [\"http://*.spin.internal\"]", component.id()));
                            }
                            return  Err(anyhow::anyhow!(
                                "Selected component '{}' cannot use service chaining to unselected component: allowed_outbound_hosts = [\"http://{}.spin.internal\"]",
                                component.id(), chaining_target
                            ));
                        }
                    }
                }
            }
        }
        anyhow::Ok(())
    })?;

    Ok(())
}

/// An address is a url-like string that contains a host, a port, and an optional scheme
#[derive(Eq, Debug, Clone)]
pub struct AllowedHostConfig {
    original: String,
    scheme: SchemeConfig,
    host: HostConfig,
    port: PortConfig,
}

impl AllowedHostConfig {
    /// Try to parse the address.
    ///
    /// If the parsing fails, the address is prepended with the scheme and parsing
    /// is tried again.
    pub fn parse(url: impl Into<String>) -> anyhow::Result<Self> {
        let original = url.into();
        let url = original.trim();
        let (scheme, rest) = url.split_once("://").with_context(|| {
            format!("{url:?} does not contain a scheme (e.g., 'http://' or '*://')")
        })?;
        let (host, rest) = rest.rsplit_once(':').unwrap_or((rest, ""));
        let port = match rest.split_once('/') {
            Some((port, path)) => {
                if !path.is_empty() {
                    bail!("{url:?} has a path but is not allowed to");
                }
                port
            }
            None => rest,
        };

        Ok(Self {
            scheme: SchemeConfig::parse(scheme)?,
            host: HostConfig::parse(host)?,
            port: PortConfig::parse(port, scheme)?,
            original,
        })
    }

    pub fn scheme(&self) -> &SchemeConfig {
        &self.scheme
    }

    pub fn host(&self) -> &HostConfig {
        &self.host
    }

    pub fn port(&self) -> &PortConfig {
        &self.port
    }

    fn allows(&self, url: &OutboundUrl) -> bool {
        self.scheme.allows(&url.scheme)
            && self.host.allows(&url.host)
            && self.port.allows(url.port, &url.scheme)
    }

    fn allows_relative(&self, schemes: &[&str]) -> bool {
        schemes.iter().any(|s| self.scheme.allows(s)) && self.host.allows_relative()
    }
}

impl PartialEq for AllowedHostConfig {
    fn eq(&self, other: &Self) -> bool {
        self.scheme == other.scheme && self.host == other.host && self.port == other.port
    }
}

impl std::fmt::Display for AllowedHostConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.original)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum SchemeConfig {
    Any,
    List(Vec<String>),
}

impl SchemeConfig {
    fn parse(scheme: &str) -> anyhow::Result<Self> {
        if scheme == "*" {
            return Ok(Self::Any);
        }

        if scheme.starts_with('{') {
            // TODO:
            bail!("scheme lists are not yet supported")
        }

        if scheme.chars().any(|c| !c.is_alphabetic()) {
            anyhow::bail!(" scheme {scheme:?} contains non alphabetic character");
        }

        Ok(Self::List(vec![scheme.into()]))
    }

    pub fn allows_any(&self) -> bool {
        matches!(self, Self::Any)
    }

    fn allows(&self, scheme: &str) -> bool {
        match self {
            SchemeConfig::Any => true,
            SchemeConfig::List(l) => l.iter().any(|s| s.as_str() == scheme),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HostConfig {
    Any,
    AnySubdomain(String),
    ToSelf,
    List(Vec<String>),
    Cidr(ipnet::IpNet),
}

impl HostConfig {
    fn parse(mut host: &str) -> anyhow::Result<Self> {
        host = host.trim();
        if host == "*" {
            return Ok(Self::Any);
        }

        if host == "self" {
            return Ok(Self::ToSelf);
        }

        if host.starts_with('{') {
            ensure!(host.ends_with('}'));
            bail!("host lists are not yet supported")
        }

        if let Ok(net) = host.parse::<ipnet::IpNet>() {
            return Ok(Self::Cidr(net));
        }

        if matches!(host.split('/').nth(1), Some(path) if !path.is_empty()) {
            bail!("hosts must not contain paths");
        }

        if let Some(domain) = host.strip_prefix("*.") {
            if domain.contains('*') {
                bail!("Invalid allowed host {host}: wildcards are allowed only as prefixes");
            }
            return Ok(Self::AnySubdomain(format!(".{domain}")));
        }

        if host.contains('*') {
            bail!("Invalid allowed host {host}: wildcards are allowed only as subdomains");
        }

        // Remove trailing slashes
        host = host.trim_end_matches('/');

        Ok(Self::List(vec![host.into()]))
    }

    fn allows(&self, host: &str) -> bool {
        match self {
            HostConfig::Any => true,
            HostConfig::AnySubdomain(suffix) => host.ends_with(suffix),
            HostConfig::List(l) => l.iter().any(|h| h.as_str() == host),
            HostConfig::ToSelf => false,
            HostConfig::Cidr(c) => {
                let Ok(ip) = host.parse::<std::net::IpAddr>() else {
                    return false;
                };
                c.contains(&ip)
            }
        }
    }

    fn allows_relative(&self) -> bool {
        matches!(self, Self::Any | Self::ToSelf)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PortConfig {
    Any,
    List(Vec<IndividualPortConfig>),
}

impl PortConfig {
    fn parse(port: &str, scheme: &str) -> anyhow::Result<PortConfig> {
        if port.is_empty() {
            return well_known_port(scheme)
                .map(|p| PortConfig::List(vec![IndividualPortConfig::Port(p)]))
                .with_context(|| format!("no port was provided and the scheme {scheme:?} does not have a known default port number"));
        }
        if port == "*" {
            return Ok(PortConfig::Any);
        }

        if port.starts_with('{') {
            // TODO:
            bail!("port lists are not yet supported")
        }

        let port = IndividualPortConfig::parse(port)?;

        Ok(Self::List(vec![port]))
    }

    fn allows(&self, port: Option<u16>, scheme: &str) -> bool {
        match self {
            PortConfig::Any => true,
            PortConfig::List(l) => {
                let port = match port.or_else(|| well_known_port(scheme)) {
                    Some(p) => p,
                    None => return false,
                };
                l.iter().any(|p| p.allows(port))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IndividualPortConfig {
    Port(u16),
    Range(Range<u16>),
}

impl IndividualPortConfig {
    fn parse(port: &str) -> anyhow::Result<Self> {
        if let Some((start, end)) = port.split_once("..") {
            let start = start
                .parse()
                .with_context(|| format!("port range {port:?} contains non-number"))?;
            let end = end
                .parse()
                .with_context(|| format!("port range {port:?} contains non-number"))?;
            return Ok(Self::Range(start..end));
        }
        Ok(Self::Port(port.parse().with_context(|| {
            format!("port {port:?} is not a number")
        })?))
    }

    fn allows(&self, port: u16) -> bool {
        match self {
            IndividualPortConfig::Port(p) => p == &port,
            IndividualPortConfig::Range(r) => r.contains(&port),
        }
    }
}

fn well_known_port(scheme: &str) -> Option<u16> {
    match scheme {
        "postgres" => Some(5432),
        "mysql" => Some(3306),
        "redis" => Some(6379),
        "mqtt" => Some(1883),
        "http" => Some(80),
        "https" => Some(443),
        _ => None,
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AllowedHostsConfig {
    All,
    SpecificHosts(Vec<AllowedHostConfig>),
}

enum PartialAllowedHostConfig {
    Exact(AllowedHostConfig),
    Unresolved(spin_expressions::Template),
}

impl PartialAllowedHostConfig {
    fn resolve(
        self,
        resolver: &spin_expressions::PreparedResolver,
    ) -> anyhow::Result<AllowedHostConfig> {
        match self {
            Self::Exact(h) => Ok(h),
            Self::Unresolved(t) => AllowedHostConfig::parse(resolver.resolve_template(&t)?),
        }
    }
}

impl AllowedHostsConfig {
    pub fn parse<S: AsRef<str>>(
        hosts: &[S],
        resolver: &spin_expressions::PreparedResolver,
    ) -> anyhow::Result<AllowedHostsConfig> {
        let partial = Self::parse_partial(hosts)?;
        let allowed = partial
            .into_iter()
            .map(|p| p.resolve(resolver))
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Self::SpecificHosts(allowed))
    }

    pub fn validate<S: AsRef<str>>(hosts: &[S]) -> anyhow::Result<()> {
        _ = Self::parse_partial(hosts)?;
        Ok(())
    }

    // Parses literals but defers templates. This is pulled out so that `validate`
    // doesn't have to deal with resolving templates.
    fn parse_partial<S: AsRef<str>>(hosts: &[S]) -> anyhow::Result<Vec<PartialAllowedHostConfig>> {
        if hosts.len() == 1 && hosts[0].as_ref() == "insecure:allow-all" {
            bail!("'insecure:allow-all' is not allowed - use '*://*:*' instead if you really want to allow all outbound traffic'")
        }
        let mut allowed = Vec::with_capacity(hosts.len());
        for host in hosts {
            let template = spin_expressions::Template::new(host.as_ref())?;
            if template.is_literal() {
                allowed.push(PartialAllowedHostConfig::Exact(AllowedHostConfig::parse(
                    host.as_ref(),
                )?));
            } else {
                allowed.push(PartialAllowedHostConfig::Unresolved(template));
            }
        }
        Ok(allowed)
    }

    /// Determine if the supplied url is allowed
    pub fn allows(&self, url: &OutboundUrl) -> bool {
        match self {
            AllowedHostsConfig::All => true,
            AllowedHostsConfig::SpecificHosts(hosts) => hosts.iter().any(|h| h.allows(url)),
        }
    }

    pub fn allows_relative_url(&self, schemes: &[&str]) -> bool {
        match self {
            AllowedHostsConfig::All => true,
            AllowedHostsConfig::SpecificHosts(hosts) => {
                hosts.iter().any(|h| h.allows_relative(schemes))
            }
        }
    }
}

impl Default for AllowedHostsConfig {
    fn default() -> Self {
        Self::SpecificHosts(Vec::new())
    }
}

/// A parsed URL used for outbound networking.
#[derive(Debug, Clone)]
pub struct OutboundUrl {
    scheme: String,
    host: String,
    port: Option<u16>,
    original: String,
}

impl OutboundUrl {
    /// Parse a URL.
    ///
    /// If parsing `url` fails, `{scheme}://` is prepended to `url` and parsing is tried again.
    pub fn parse(url: impl Into<String>, scheme: &str) -> anyhow::Result<Self> {
        let mut url = url.into();
        let original = url.clone();

        // Ensure that the authority is url encoded. Since the authority is ignored after this,
        // we can always url encode the authority even if it is already encoded.
        if let Some(at) = url.find('@') {
            let scheme_end = url.find("://").map(|e| e + 3).unwrap_or(0);
            let path_start = url[scheme_end..]
                .find('/') // This can calculate the wrong index if the username or password contains a '/'
                .map(|e| e + scheme_end)
                .unwrap_or(usize::MAX);

            if at < path_start {
                let userinfo = &url[scheme_end..at];

                let encoded = urlencoding::encode(userinfo);
                let prefix = &url[..scheme_end];
                let suffix = &url[scheme_end + userinfo.len()..];
                url = format!("{prefix}{encoded}{suffix}");
            }
        }

        let parsed = match url::Url::parse(&url) {
            Ok(url) if url.has_host() => Ok(url),
            first_try => {
                let second_try: anyhow::Result<url::Url> = format!("{scheme}://{url}")
                    .as_str()
                    .try_into()
                    .context("could not convert into a url");
                match (second_try, first_try.map_err(|e| e.into())) {
                    (Ok(u), _) => Ok(u),
                    // Return an error preferring the error from the first attempt if present
                    (_, Err(e)) | (Err(e), _) => Err(e),
                }
            }
        }?;

        Ok(Self {
            scheme: parsed.scheme().to_owned(),
            host: parsed
                .host_str()
                .with_context(|| format!("{url:?} does not have a host component"))?
                .to_owned(),
            port: parsed.port(),
            original,
        })
    }

    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    pub fn authority(&self) -> String {
        if let Some(port) = self.port {
            format!("{}:{port}", self.host)
        } else {
            self.host.clone()
        }
    }
}

impl std::fmt::Display for OutboundUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.original)
    }
}

pub fn is_service_chaining_host(host: &str) -> bool {
    parse_service_chaining_host(host).is_some()
}

pub fn parse_service_chaining_target(url: &http::Uri) -> Option<String> {
    let host = url.authority().map(|a| a.host().trim())?;
    parse_service_chaining_host(host)
}

fn parse_service_chaining_host(host: &str) -> Option<String> {
    let (host, _) = host.rsplit_once(':').unwrap_or((host, ""));

    let (first, rest) = host.split_once('.')?;

    if rest == SERVICE_CHAINING_DOMAIN {
        Some(first.to_owned())
    } else {
        None
    }
}

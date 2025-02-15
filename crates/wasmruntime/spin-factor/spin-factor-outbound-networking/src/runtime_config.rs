use std::{collections::HashMap, str::FromStr, sync::Arc};

use anyhow::{ensure, Context};
use rustls::{ClientConfig, RootCertStore};
use rustls_pki_types::{CertificateDer, PrivateKeyDer};

/// Runtime configuration for outbound networking.
#[derive(Debug)]
pub struct RuntimeConfig {
    /// Maps component ID -> HostClientConfigs
    component_host_client_configs: HashMap<String, HostClientConfigs>,
    /// The default [`ClientConfig`] for a host if one is not explicitly configured for it.
    default_client_config: Arc<ClientConfig>,
}

// Maps host authority -> ClientConfig
type HostClientConfigs = Arc<HashMap<String, Arc<ClientConfig>>>;

impl RuntimeConfig {
    /// Returns runtime config with the given list of [`TlsConfig`]s. The first
    /// [`TlsConfig`] to match an outgoing request (based on
    /// [`TlsConfig::components`] and [`TlsConfig::hosts`]) will be used.
    pub fn new(tls_configs: impl IntoIterator<Item = TlsConfig>) -> anyhow::Result<Self> {
        let mut component_host_client_configs = HashMap::<String, HostClientConfigs>::new();
        for tls_config in tls_configs {
            ensure!(
                !tls_config.components.is_empty(),
                "client TLS 'components' list may not be empty"
            );
            ensure!(
                !tls_config.hosts.is_empty(),
                "client TLS 'hosts' list may not be empty"
            );
            let client_config = Arc::new(
                tls_config
                    .to_client_config()
                    .context("error building TLS client config")?,
            );
            for component in &tls_config.components {
                let host_configs = component_host_client_configs
                    .entry(component.clone())
                    .or_default();
                for host in &tls_config.hosts {
                    validate_host(host)?;
                    // First matching (component, host) pair wins
                    Arc::get_mut(host_configs)
                        .unwrap()
                        .entry(host.clone())
                        .or_insert_with(|| client_config.clone());
                }
            }
        }

        let default_client_config = Arc::new(TlsConfig::default().to_client_config()?);

        Ok(Self {
            component_host_client_configs,
            default_client_config,
        })
    }

    /// Returns [`ComponentTlsConfigs`] for the given component.
    pub fn get_component_tls_configs(&self, component_id: &str) -> ComponentTlsConfigs {
        let host_client_configs = self
            .component_host_client_configs
            .get(component_id)
            .cloned();
        ComponentTlsConfigs {
            host_client_configs,
            default_client_config: self.default_client_config.clone(),
        }
    }

    /// Returns a [`ClientConfig`] for the given component and host authority.
    ///
    /// This is a convenience method, equivalent to:
    ///  `.get_client_config(component_id).get_client_config(host)`
    pub fn get_client_config(&self, component_id: &str, host: &str) -> Arc<ClientConfig> {
        let component_config = self.get_component_tls_configs(component_id);
        component_config.get_client_config(host).clone()
    }
}

pub(crate) fn validate_host(host: &str) -> anyhow::Result<()> {
    // Validate hostname
    let authority = http::uri::Authority::from_str(host)
        .with_context(|| format!("invalid TLS 'host' {host:?}"))?;
    ensure!(
        authority.port().is_none(),
        "invalid TLS 'host' {host:?}; ports not currently supported"
    );
    Ok(())
}

/// TLS configurations for a specific component.
#[derive(Clone)]
pub struct ComponentTlsConfigs {
    host_client_configs: Option<HostClientConfigs>,
    default_client_config: Arc<ClientConfig>,
}

impl ComponentTlsConfigs {
    /// Returns a [`ClientConfig`] for the given host authority.
    pub fn get_client_config(&self, host: &str) -> &Arc<ClientConfig> {
        self.host_client_configs
            .as_ref()
            .and_then(|configs| configs.get(host))
            .unwrap_or(&self.default_client_config)
    }
}

#[derive(Debug)]
pub struct ClientCertConfig {
    cert_chain: Vec<CertificateDer<'static>>,
    key_der: PrivateKeyDer<'static>,
}

/// TLS configuration for one or more component(s) and host(s).
#[derive(Debug)]
pub struct TlsConfig {
    /// The component(s) this configuration applies to.
    pub components: Vec<String>,
    /// The host(s) this configuration applies to.
    pub hosts: Vec<String>,
    /// A set of CA certs that should be considered valid roots.
    pub root_certificates: Vec<rustls_pki_types::CertificateDer<'static>>,
    /// If true, the "standard" CA certs defined by `webpki-roots` crate will be
    /// considered valid roots in addition to `root_certificates`.
    pub use_webpki_roots: bool,
    /// A certificate and private key to be used as the client certificate for
    /// "mutual TLS" (mTLS).
    pub client_cert: Option<ClientCertConfig>,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            components: vec![],
            hosts: vec![],
            root_certificates: vec![],
            // Use webpki roots by default
            use_webpki_roots: true,
            client_cert: None,
        }
    }
}

impl TlsConfig {
    fn to_client_config(&self) -> anyhow::Result<ClientConfig> {
        let mut root_store = RootCertStore::empty();
        if self.use_webpki_roots {
            root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        }
        for ca in &self.root_certificates {
            root_store.add(ca.clone())?;
        }

        let builder = ClientConfig::builder().with_root_certificates(root_store);

        if let Some(ClientCertConfig {
            cert_chain,
            key_der,
        }) = &self.client_cert
        {
            Ok(builder.with_client_auth_cert(cert_chain.clone(), key_der.clone_key())?)
        } else {
            Ok(builder.with_no_client_auth())
        }
    }
}

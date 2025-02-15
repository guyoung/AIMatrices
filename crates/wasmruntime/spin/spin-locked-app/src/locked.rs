//! Spin lock file (spin.lock) serialization models.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use spin_serde::{DependencyName, FixedVersionBackwardCompatible};
use std::collections::BTreeMap;

use crate::{
    metadata::MetadataExt,
    values::{ValuesMap, ValuesMapBuilder},
};

/// A String-keyed map with deterministic serialization order.
pub type LockedMap<T> = std::collections::BTreeMap<String, T>;

/// If present and required in `host_requirements`, the host must support
/// local service chaining (*.spin.internal) or reject the app.
pub const SERVICE_CHAINING_KEY: &str = "local_service_chaining";

/// Indicates that a host feature is optional. This is the default and is
/// equivalent to omitting the feature from `host_requirements`.
pub const HOST_REQ_OPTIONAL: &str = "optional";
/// Indicates that a host feature is required.
pub const HOST_REQ_REQUIRED: &str = "required";

/// Identifies fields in the LockedApp that the host must process if present.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MustUnderstand {
    /// If present in `must_understand`, the host must support all features
    /// in the app's `host_requirements` section.
    HostRequirements,
}

/// Features or capabilities the application requires the host to support.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostRequirement {
    /// The application requires local service chaining.
    LocalServiceChaining,
}

/// A LockedApp represents a "fully resolved" Spin application.
#[derive(Clone, Debug, Deserialize)]
pub struct LockedApp {
    /// Locked schema version
    pub lock_version: FixedVersionBackwardCompatible<1>,
    /// Identifies fields in the LockedApp that the host must process if present.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub must_understand: Vec<MustUnderstand>,
    /// Application metadata
    #[serde(default, skip_serializing_if = "ValuesMap::is_empty")]
    pub metadata: ValuesMap,
    /// Host requirements
    #[serde(
        default,
        skip_serializing_if = "ValuesMap::is_empty",
        deserialize_with = "deserialize_host_requirements"
    )]
    pub host_requirements: ValuesMap,
    /// Custom config variables
    #[serde(default, skip_serializing_if = "LockedMap::is_empty")]
    pub variables: LockedMap<Variable>,
    /// Application triggers
    pub triggers: Vec<LockedTrigger>,
    /// Application components
    pub components: Vec<LockedComponent>,
}

fn deserialize_host_requirements<'de, D>(deserializer: D) -> Result<ValuesMap, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct HostRequirementsVisitor;
    impl<'de> serde::de::Visitor<'de> for HostRequirementsVisitor {
        type Value = ValuesMap;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("struct ValuesMap")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            use serde::de::Error;

            let mut hr = ValuesMapBuilder::new();

            while let Some(key) = map.next_key::<String>()? {
                let value: serde_json::Value = map.next_value()?;
                if value.as_str() == Some(HOST_REQ_OPTIONAL) {
                    continue;
                }

                hr.serializable(key, value).map_err(A::Error::custom)?;
            }

            Ok(hr.build())
        }
    }
    let m = deserializer.deserialize_map(HostRequirementsVisitor)?;
    let unsupported: Vec<_> = m
        .keys()
        .filter(|k| !SUPPORTED_HOST_REQS.contains(&k.as_str()))
        .map(|k| k.to_string())
        .collect();
    if unsupported.is_empty() {
        Ok(m)
    } else {
        let msg = format!("This version of Spin does not support the following features required by this application: {}", unsupported.join(", "));
        Err(serde::de::Error::custom(msg))
    }
}

const SUPPORTED_HOST_REQS: &[&str] = &[SERVICE_CHAINING_KEY];

impl Serialize for LockedApp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let version = if self.must_understand.is_empty() && self.host_requirements.is_empty() {
            0
        } else {
            1
        };

        let mut la = serializer.serialize_struct("LockedApp", 7)?;
        la.serialize_field("lock_version", &version)?;
        if !self.must_understand.is_empty() {
            la.serialize_field("must_understand", &self.must_understand)?;
        }
        if !self.metadata.is_empty() {
            la.serialize_field("metadata", &self.metadata)?;
        }
        if !self.host_requirements.is_empty() {
            la.serialize_field("host_requirements", &self.host_requirements)?;
        }
        if !self.variables.is_empty() {
            la.serialize_field("variables", &self.variables)?;
        }
        la.serialize_field("triggers", &self.triggers)?;
        la.serialize_field("components", &self.components)?;
        la.end()
    }
}

impl LockedApp {
    /// Deserializes a [`LockedApp`] from the given JSON data.
    pub fn from_json(contents: &[u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(contents)
    }

    /// Serializes the [`LockedApp`] into JSON data.
    pub fn to_json(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec_pretty(&self)
    }

    /// Deserializes typed metadata for this app.
    ///
    /// Returns `Ok(None)` if there is no metadata for the given `key` and an
    /// `Err` only if there _is_ a value for the `key` but the typed
    /// deserialization failed.
    pub fn get_metadata<'this, T: Deserialize<'this>>(
        &'this self,
        key: crate::MetadataKey<T>,
    ) -> crate::Result<Option<T>> {
        self.metadata.get_typed(key)
    }

    /// Deserializes typed metadata for this app.
    ///
    /// Like [`LockedApp::get_metadata`], but returns an error if there is
    /// no metadata for the given `key`.
    pub fn require_metadata<'this, T: Deserialize<'this>>(
        &'this self,
        key: crate::MetadataKey<T>,
    ) -> crate::Result<T> {
        self.metadata.require_typed(key)
    }

    /// Checks that the application does not have any host requirements
    /// outside the supported set. The error case returns a comma-separated
    /// list of unmet requirements.
    pub fn ensure_needs_only(&self, supported: &[&str]) -> Result<(), String> {
        let unmet_requirements = self
            .host_requirements
            .keys()
            .filter(|hr| !supported.contains(&hr.as_str()))
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        if unmet_requirements.is_empty() {
            Ok(())
        } else {
            let message = unmet_requirements.join(", ");
            Err(message)
        }
    }
}

/// A LockedComponent represents a "fully resolved" Spin component.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LockedComponent {
    /// Application-unique component identifier
    pub id: String,
    /// Component metadata
    #[serde(default, skip_serializing_if = "ValuesMap::is_empty")]
    pub metadata: ValuesMap,
    /// Wasm source
    pub source: LockedComponentSource,
    /// WASI environment variables
    #[serde(default, skip_serializing_if = "LockedMap::is_empty")]
    pub env: LockedMap<String>,
    /// WASI filesystem contents
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<ContentPath>,
    /// Custom config values
    #[serde(default, skip_serializing_if = "LockedMap::is_empty")]
    pub config: LockedMap<String>,
    /// Component dependencies
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub dependencies: BTreeMap<DependencyName, LockedComponentDependency>,
}

/// A LockedDependency represents a "fully resolved" Spin component dependency.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LockedComponentDependency {
    /// Locked dependency source
    pub source: LockedComponentSource,
    /// The specific export to use from the dependency, if any.
    pub export: Option<String>,
    /// Which configurations to inherit from parent
    #[serde(default, skip_serializing_if = "InheritConfiguration::is_none")]
    pub inherit: InheritConfiguration,
}

/// InheritConfiguration specifies which configurations to inherit from parent.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InheritConfiguration {
    /// Dependencies will inherit all configurations from parent.
    All,
    /// Dependencies will inherit only the specified configurations from parent
    /// (if empty then deny-all is enforced).
    Some(Vec<String>),
}

impl Default for InheritConfiguration {
    fn default() -> Self {
        InheritConfiguration::Some(vec![])
    }
}

impl InheritConfiguration {
    fn is_none(&self) -> bool {
        matches!(self, InheritConfiguration::Some(configs) if configs.is_empty())
    }
}

/// A LockedComponentSource specifies a Wasm source.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LockedComponentSource {
    /// Wasm source content type (e.g. "application/wasm")
    pub content_type: String,
    /// Wasm source content specification
    #[serde(flatten)]
    pub content: ContentRef,
}

/// A ContentPath specifies content mapped to a WASI path.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentPath {
    /// Content specification
    #[serde(flatten)]
    pub content: ContentRef,
    /// WASI mount path
    pub path: PathBuf,
}

/// A ContentRef represents content used by an application.
///
/// At least one of `source`, `inline`, or `digest` must be specified. Implementations may
/// require one or the other (or both).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ContentRef {
    /// A URI where the content can be accessed. Implementations may support
    /// different URI schemes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The content itself, base64-encoded.
    ///
    /// NOTE: This is both an optimization for small content and a workaround
    /// for certain OCI implementations that don't support 0 or 1 byte blobs.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "spin_serde::base64"
    )]
    pub inline: Option<Vec<u8>>,
    /// If set, the content must have the given SHA-256 digest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
}

/// A LockedTrigger specifies configuration for an application trigger.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LockedTrigger {
    /// Application-unique trigger identifier
    pub id: String,
    /// Trigger type (e.g. "http")
    pub trigger_type: String,
    /// Trigger-type-specific configuration
    pub trigger_config: Value,
}

/// A Variable specifies a custom configuration variable.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Variable {
    /// The variable's default value. If unset, the variable is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// If set, the variable's value may be sensitive and e.g. shouldn't be logged.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub secret: bool,
}

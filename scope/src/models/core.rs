use crate::models::{HelpMetadata, ScopeModel};
use derive_builder::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const FILE_PATH_ANNOTATION: &str = "scope.github.com/file-path";
pub const FILE_DIR_ANNOTATION: &str = "scope.github.com/file-dir";
pub const FILE_EXEC_PATH_ANNOTATION: &str = "scope.github.com/bin-path";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Builder, JsonSchema)]
pub struct ModelMetadataAnnotations {
    #[serde(rename = "scope.github.com/file-path")]
    #[schemars(skip)]
    /// File path for the resource, generated automatically.
    pub file_path: Option<String>,

    #[serde(rename = "scope.github.com/file-dir")]
    #[schemars(skip)]
    /// Directory containing the resource, generated automatically.
    pub file_dir: Option<String>,

    /// The current working directory of the scope command, generated automatically.
    pub working_dir: Option<String>,

    #[serde(rename = "scope.github.com/bin-path")]
    /// When running commands, additional paths that should be paced at the _beginning_ of the `PATH`.
    pub bin_path: Option<String>,

    #[serde(flatten)]
    pub extra: BTreeMap<String, String>,
}

impl ModelMetadata {
    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn description(&self) -> String {
        self.description.to_string()
    }

    pub fn file_path(&self) -> String {
        match &self.annotations.file_path {
            Some(v) => v.to_string(),
            None => "unknown".to_string(),
        }
    }

    pub fn containing_dir(&self) -> String {
        match &self.annotations.file_dir {
            Some(v) => v.to_string(),
            None => "unknown".to_string(),
        }
    }

    pub fn exec_path(&self) -> String {
        match &self.annotations.bin_path {
            Some(v) => {
                format!(
                    "{}:{}",
                    v,
                    std::env::var("PATH").unwrap_or_else(|_| "".to_string())
                )
            }
            None => std::env::var("PATH").unwrap_or_else(|_| "".to_string()),
        }
    }

    pub fn new(name: &str) -> ModelMetadata {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Builder, JsonSchema)]
#[builder(setter(into))]
pub struct ModelMetadata {
    /// Name of the resource, needs to be unique across `kinds`. When two resources share a name,
    /// the one "closest" to the current working directory will take precedence.
    pub name: String,

    #[serde(default = "default_description")]
    /// Description of this resource, used when listing resources and helpful to inform users why
    /// the resource exists.
    pub description: String,

    #[serde(default)]
    /// Annotations attach arbitrary non-identifying metadata to objects.
    pub annotations: ModelMetadataAnnotations,

    #[serde(default)]
    /// Key/value pairs, allows resources to be easily filtered from the CLI.
    pub labels: BTreeMap<String, String>,
}

fn default_description() -> String {
    "Description not provided".to_string()
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(setter(into))]
#[serde(rename_all = "camelCase")]
pub struct ModelRoot<V> {
    pub api_version: String,
    pub kind: String,
    pub metadata: ModelMetadata,
    pub spec: V,
}

impl<S> HelpMetadata for ModelRoot<S> {
    fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }

    fn full_name(&self) -> String {
        format!("{}/{}", self.kind, self.name())
    }
}

impl<S> ScopeModel<S> for ModelRoot<S> {
    fn api_version(&self) -> String {
        self.api_version.to_string()
    }

    fn kind(&self) -> String {
        self.kind.to_string()
    }

    fn spec(&self) -> &S {
        &self.spec
    }
}

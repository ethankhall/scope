use crate::core::ModelMetadata;
use crate::v1alpha::V1AlphaApiVersion;
use derive_builder::Builder;
use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReportDestinationGithubIssueSpec {
    pub owner: String,
    pub repo: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ReportDestinationSpec {
    RustyPaste { url: String },
    GithubIssue(ReportDestinationGithubIssueSpec),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ReportLocationSpec {
    #[serde(with = "serde_yaml::with::singleton_map")]
    #[schemars(with = "ReportDestinationSpec")]
    pub destination: ReportDestinationSpec,
}

#[derive(Serialize, Deserialize, Debug, strum::Display, Clone, PartialEq, JsonSchema)]
pub enum ReportLocationKind {
    #[strum(serialize = "ScopeReportLocation")]
    ScopeReportLocation,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Builder, JsonSchema)]
#[builder(setter(into))]
#[serde(rename_all = "camelCase")]
pub struct V1AlphaReportLocation {
    pub api_version: V1AlphaApiVersion,
    pub kind: ReportLocationKind,
    pub metadata: ModelMetadata,
    pub spec: ReportLocationSpec,
}

impl crate::ScopeModel<ReportLocationSpec> for V1AlphaReportLocation {
    fn api_version(&self) -> String {
        <Self as crate::InternalScopeModel<_>>::int_api_version()
    }

    fn kind(&self) -> String {
        <Self as crate::InternalScopeModel<_>>::int_kind()
    }

    fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }

    fn spec(&self) -> &ReportLocationSpec {
        &self.spec
    }
}

impl crate::InternalScopeModel<ReportLocationSpec> for V1AlphaReportLocation {
    fn int_api_version() -> String {
        V1AlphaApiVersion::ScopeV1Alpha.to_string()
    }

    fn int_kind() -> String {
        ReportLocationKind::ScopeReportLocation.to_string()
    }
    #[cfg(test)]
    fn examples() -> Vec<String> {
        vec![
            "v1alpha/ReportLocation.github.yaml".to_string(),
            "v1alpha/ReportLocation.rustyPaste.yaml".to_string(),
        ]
    }
}

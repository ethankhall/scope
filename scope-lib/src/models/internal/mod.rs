use crate::models::ModelRoot;
use serde_yaml::Value;

mod doctor_exec;
mod doctor_setup;
mod known_error;
mod report_definition;
mod upload_location;

use self::doctor_exec::DoctorExec;
use self::doctor_setup::DoctorSetup;
use self::known_error::KnownError;
use self::report_definition::ReportDefinition;
use self::upload_location::ReportUploadLocation;

pub mod prelude {
    pub use super::ParsedConfig;
    pub use super::{
        doctor_exec::*, doctor_setup::*, known_error::*, report_definition::*, upload_location::*,
    };
}

#[derive(Debug, PartialEq)]
pub enum ParsedConfig {
    DoctorCheck(ModelRoot<DoctorExec>),
    KnownError(ModelRoot<KnownError>),
    ReportUpload(ModelRoot<ReportUploadLocation>),
    ReportDefinition(ModelRoot<ReportDefinition>),
    DoctorSetup(ModelRoot<DoctorSetup>),
}

#[cfg(test)]
impl ParsedConfig {
    pub fn get_report_upload_spec(&self) -> Option<ReportUploadLocation> {
        match self {
            ParsedConfig::ReportUpload(root) => Some(root.spec.clone()),
            _ => None,
        }
    }

    pub fn get_report_def_spec(&self) -> Option<ReportDefinition> {
        match self {
            ParsedConfig::ReportDefinition(root) => Some(root.spec.clone()),
            _ => None,
        }
    }

    pub fn get_known_error_spec(&self) -> Option<KnownError> {
        match self {
            ParsedConfig::KnownError(root) => Some(root.spec.clone()),
            _ => None,
        }
    }

    pub fn get_doctor_check_spec(&self) -> Option<DoctorExec> {
        match self {
            ParsedConfig::DoctorCheck(root) => Some(root.spec.clone()),
            _ => None,
        }
    }

    pub fn get_doctor_setup_spec(&self) -> Option<DoctorSetup> {
        match self {
            ParsedConfig::DoctorSetup(root) => Some(root.spec.clone()),
            _ => None,
        }
    }
}

impl TryFrom<ModelRoot<Value>> for ParsedConfig {
    type Error = anyhow::Error;

    fn try_from(value: ModelRoot<Value>) -> Result<Self, Self::Error> {
        ParsedConfig::try_from(&value)
    }
}
use crate::shared::HelpMetadata;
use derivative::Derivative;
use regex::Regex;
use dev_scope_model::prelude::{ModelMetadata, V1AlphaKnownError};

#[derive(Derivative)]
#[derivative(PartialEq)]
#[derive(Debug, Clone)]
pub struct KnownError {
    pub metadata: ModelMetadata,
    pub description: String,
    pub pattern: String,
    #[derivative(PartialEq = "ignore")]
    pub regex: Regex,
    pub help_text: String,
}

impl HelpMetadata for KnownError {
    fn description(&self) -> &str {
        &self.description
    }

    fn name(&self) -> &str {
        &self.metadata.name
    }
}

impl TryFrom<V1AlphaKnownError> for KnownError {
    type Error = anyhow::Error;

    fn try_from(value: V1AlphaKnownError) -> Result<Self, Self::Error> {
        let regex = Regex::new(&value.spec.pattern)?;
        Ok(KnownError {
            metadata: value.metadata,
            pattern: value.spec.pattern,
            regex,
            help_text: value.spec.help,
            description: value.spec.description,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::models::parse_models_from_string;
    use crate::shared::models::prelude::KnownError;
    use regex::Regex;
    use std::path::Path;

    #[test]
    fn test_parse_scope_known_error() {
        let text = "apiVersion: scope.github.com/v1alpha
kind: ScopeKnownError
metadata:
  name: error-exists
spec:
  description: Check if the word error is in the logs
  pattern: error
  help: The command had an error, try reading the logs around there to find out what happened.";

        let path = Path::new("/foo/bar/file.yaml");
        let configs = parse_models_from_string(path, text).unwrap();
        assert_eq!(1, configs.len());
        assert_eq!(configs[0].get_known_error_spec().unwrap(), KnownError {
            description: "Check if the word error is in the logs".to_string(),
            help_text: "The command had an error, try reading the logs around there to find out what happened.".to_string(),
            pattern: "error".to_string(),
            regex: Regex::new("error").unwrap()
        });
    }
}

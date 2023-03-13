use crate::impl_json_property;
use crate::impl_string_property;
use crate::impl_uuid_property;
use anyhow::Result;
use getset::Getters;
use getset::Setters;
use serde_json::Value as Json;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectId {
    value: Uuid,
}

impl_uuid_property!(ProjectId);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct ProjectName {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(ProjectName);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct ProjectDescription {
    #[validate(length(min = 0))]
    value: String,
}

impl_string_property!(ProjectDescription);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectConfig {
    value: Json,
}

impl_json_property!(ProjectConfig);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, serde::Serialize)]
pub struct Project {
    #[getset(get = "pub")]
    id: ProjectId,
    #[getset(get = "pub", set = "pub")]
    name: ProjectName,
    #[getset(get = "pub", set = "pub")]
    description: ProjectDescription,
    #[getset(get = "pub", set = "pub")]
    config: Option<ProjectConfig>,
}

impl Project {
    pub fn new(
        id: String,
        name: String,
        description: String,
        config: impl Into<Option<Json>>,
    ) -> Result<Self> {
        Ok(Self {
            id: ProjectId::try_from(id)?,
            name: ProjectName::new(name)?,
            description: ProjectDescription::new(description)?,
            config: config.into().map(|json| ProjectConfig::new(json)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_project_id() {
        assert!(matches!(
            ProjectId::try_from(testutils::rand::uuid()),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_project_id() {
        assert!(matches!(
            ProjectId::try_from(testutils::rand::string(255)),
            Err(_)
        ));
    }

    #[test]
    fn test_valid_project_name() {
        assert!(matches!(
            ProjectName::new(testutils::rand::string(255)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_project_name() {
        assert!(matches!(ProjectName::new(""), Err(_)));
    }

    #[test]
    fn test_valid_project_description() {
        assert!(matches!(
            ProjectDescription::new(testutils::rand::string(255)),
            Ok(_)
        ));
        assert!(matches!(ProjectDescription::new(""), Ok(_)));
    }
}

use crate::impl_json_property;
use crate::impl_string_property;
use crate::impl_uuid_property;
use anyhow::Result;
use getset::Getters;
use getset::Setters;
use serde_json::json;
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

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
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
        config: Option<Json>,
    ) -> Result<Self> {
        Ok(Self {
            id: ProjectId::try_from(id)?,
            name: ProjectName::new(name)?,
            description: ProjectDescription::new(description)?,
            config: config.map(|json| ProjectConfig::new(json)),
        })
    }
}

impl TryFrom<Json> for Project {
    type Error = anyhow::Error;

    fn try_from(json: Json) -> std::result::Result<Self, Self::Error> {
        let id = json!(Uuid::new_v4().to_string());
        let id = match &json["id"] {
            Json::Null => &id,
            value => value,
        };
        let config = match &json["config"] {
            Json::Null => None,
            value => Some(value),
        };
        Ok(Self {
            id: ProjectId::try_from(id)?,
            name: ProjectName::try_from(&json["name"])?,
            description: ProjectDescription::try_from(&json["description"])?,
            config: config.map(|json| ProjectConfig::new(json.clone())),
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

    #[test]
    fn test_valid_project_config() {
        let json = format!(
            r#"
                {{
                    "{}": "{}",
                    "{}": {}
                }}
            "#,
            testutils::rand::string(10),
            testutils::rand::string(10),
            testutils::rand::string(10),
            testutils::rand::i64(-10, 10)
        );
        assert!(matches!(ProjectConfig::try_from(json), Ok(_)));
    }

    #[test]
    fn test_invalid_project_config() {
        let json = format!(
            r#"
                {{
                    "{}": "{}",
                    "{}": {}
                }}}}
            "#,
            testutils::rand::string(10),
            testutils::rand::string(10),
            testutils::rand::string(10),
            testutils::rand::i64(-10, 10)
        );
        assert!(matches!(ProjectConfig::try_from(json), Err(_)));
    }
}
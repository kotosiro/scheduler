use crate::impl_json_property;
use crate::impl_string_property;
use crate::impl_uuid_property;
use anyhow::Result;
use chrono::NaiveDateTime;
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
    #[getset(get = "pub", set = "pub")]
    created_at: Option<NaiveDateTime>,
    #[getset(get = "pub", set = "pub")]
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectDestructor {
    pub id: ProjectId,
    pub name: ProjectName,
    pub description: ProjectDescription,
    pub config: Option<ProjectConfig>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Project {
    pub fn new(
        id: ProjectId,
        name: ProjectName,
        description: ProjectDescription,
        config: Option<ProjectConfig>,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Result<Self> {
        Ok(Self {
            id,
            name,
            description,
            config,
            created_at,
            updated_at,
        })
    }

    pub fn destruct(mut self) -> ProjectDestructor {
        ProjectDestructor {
            id: self.id,
            name: self.name,
            description: self.description,
            config: self.config.take(),
            created_at: self.created_at.take(),
            updated_at: self.updated_at.take(),
        }
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
            testutils::rand::i32(-10, 10)
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
            testutils::rand::i32(-10, 10)
        );
        assert!(matches!(ProjectConfig::try_from(json), Err(_)));
    }
}

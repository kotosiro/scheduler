use crate::impl_string;
use anyhow::Context;
use anyhow::Result;
use chrono::NaiveDateTime;
use getset::Getters;
use getset::Setters;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectId {
    value: Uuid,
}

impl ProjectId {
    pub fn new(value: Uuid) -> Result<ProjectId> {
        Ok(ProjectId { value })
    }

    pub fn to_uuid(&self) -> Uuid {
        self.value
    }
}

impl std::fmt::Display for ProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.hyphenated())
    }
}

impl TryFrom<&str> for ProjectId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let value =
            Uuid::parse_str(value).context(format!(r#"failed to parse id "{}" as uuid"#, value))?;
        Ok(ProjectId { value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct ProjectName {
    #[validate(length(min = 1))]
    value: String,
}

impl_string!(ProjectName);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct ProjectDescription {
    #[validate(length(min = 1))]
    value: String,
}

impl_string!(ProjectDescription);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
struct Project {
    #[getset(get = "pub")]
    id: ProjectId,
    #[getset(get = "pub", set = "pub")]
    name: ProjectName,
    #[getset(get = "pub", set = "pub")]
    pub description: ProjectDescription,
    #[getset(get = "pub", set = "pub")]
    pub created_at: NaiveDateTime,
    #[getset(get = "pub", set = "pub")]
    pub updated_at: NaiveDateTime,
}

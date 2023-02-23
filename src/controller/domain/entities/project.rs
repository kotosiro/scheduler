use crate::impl_string_property;
use anyhow::Context;
use anyhow::Result;
//use chrono::NaiveDateTime;
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

impl TryFrom<String> for ProjectId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        let value = Uuid::parse_str(value.as_str())
            .context(format!(r#"failed to parse id "{}" as uuid"#, value))?;
        Ok(ProjectId { value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct ProjectName {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(ProjectName);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct ProjectDescription {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(ProjectDescription);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct Project {
    #[getset(get = "pub")]
    id: ProjectId,
    #[getset(get = "pub", set = "pub")]
    name: ProjectName,
    #[getset(get = "pub", set = "pub")]
    description: ProjectDescription,
    //    #[getset(get = "pub", set = "pub")]
    //    created_at: NaiveDateTime,
    //    #[getset(get = "pub", set = "pub")]
    //    updated_at: NaiveDateTime,
}

impl Project {
    pub fn new(
        id: ProjectId,
        name: ProjectName,
        description: ProjectDescription,
        //        created_at: NaiveDateTime,
        //        updated_at: NaiveDateTime,
    ) -> Result<Self> {
        Ok(Self {
            id,
            name,
            description,
            //            created_at,
            //            updated_at,
        })
    }
}

#[cfg(tests)]
mod test {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_valid_uuid() {
        assert!(matches!(
            ProjectId::try_from(Uuid::new_v4().to_string()),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_uuid() {
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
    }

    #[test]
    fn test_invalid_project_description() {
        assert!(matches!(ProjectDescription::new(""), Err(_)));
    }
}

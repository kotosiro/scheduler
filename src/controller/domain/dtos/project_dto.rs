use crate::controller::domain::entities::project::Project;
use crate::controller::domain::entities::project::ProjectDestructure;
use chrono::NaiveDateTime;
use serde_json::Value as Json;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProjectDto {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub config: Option<Json>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<Project> for ProjectDto {
    fn from(project: Project) -> Self {
        let ProjectDestructure {
            id,
            name,
            description,
            mut config,
            mut created_at,
            mut updated_at,
        } = project.destruct();
        Self {
            id: id.to_uuid(),
            name: name.into_string(),
            description: description.into_string(),
            config: config.take().map(|config| config.to_json()),
            created_at: created_at.take(),
            updated_at: updated_at.take(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::domain::entities::project::Project;
    use crate::controller::domain::entities::project::ProjectConfig;
    use crate::controller::domain::entities::project::ProjectDescription;
    use crate::controller::domain::entities::project::ProjectId;
    use crate::controller::domain::entities::project::ProjectName;

    #[test]
    fn test_from() {
        let id = ProjectId::new(Uuid::new_v4()).expect("cannot parse project id properly");
        let name =
            ProjectName::new(testutils::rand::string(10)).expect("failed to parse project name");
        let description = ProjectDescription::new(testutils::rand::string(10))
            .expect("failed to parse project description");
        let config = ProjectConfig::try_from(format!(
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
        ))
        .ok();
        let created_at = testutils::rand::now();
        let updated_at = testutils::rand::now();
        let project = Project::new(
            id.clone(),
            name.clone(),
            description.clone(),
            config.clone(),
            Some(created_at.clone()),
            Some(updated_at.clone()),
        )
        .expect("failed to create project");
        let dto = ProjectDto::from(project);
        assert_eq!(dto.id, id.to_uuid());
        assert_eq!(dto.name, name.into_string());
        assert_eq!(dto.description, description.into_string());
        assert_eq!(dto.config, config.map(|c| c.to_json()));
        assert_eq!(dto.created_at, Some(created_at));
        assert_eq!(dto.updated_at, Some(updated_at));
    }
}

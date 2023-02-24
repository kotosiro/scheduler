use crate::controller::domain::entities::workflow::Workflow;
use crate::controller::domain::entities::workflow::WorkflowDestructor;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WorkflowDto {
    pub id: Uuid,
    pub name: String,
    pub project_id: Uuid,
    pub description: String,
    pub paused: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<Workflow> for WorkflowDto {
    fn from(workflow: Workflow) -> Self {
        let WorkflowDestructor {
            id,
            name,
            project_id,
            description,
            paused,
            mut created_at,
            mut updated_at,
        } = workflow.destruct();
        Self {
            id: id.to_uuid(),
            name: name.into_string(),
            project_id: project_id.to_uuid(),
            description: description.into_string(),
            paused: paused.to_bool(),
            created_at: created_at.take(),
            updated_at: updated_at.take(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::domain::entities::project::ProjectId;
    use crate::controller::domain::entities::workflow::Workflow;
    use crate::controller::domain::entities::workflow::WorkflowDescription;
    use crate::controller::domain::entities::workflow::WorkflowId;
    use crate::controller::domain::entities::workflow::WorkflowName;
    use crate::controller::domain::entities::workflow::WorkflowPaused;

    #[test]
    fn test_from() {
        let id = WorkflowId::try_from(testutils::rand::uuid())
            .expect("workflow id should be parsed properly");
        let name = WorkflowName::new(testutils::rand::string(10))
            .expect("workflow name should be parsed properly");
        let project_id = ProjectId::try_from(testutils::rand::uuid())
            .expect("project id should be parsed properly");
        let description = WorkflowDescription::new(testutils::rand::string(10))
            .expect("workflow description should be parsed properly");
        let paused = WorkflowPaused::new(testutils::rand::bool());
        let created_at = testutils::rand::now();
        let updated_at = testutils::rand::now();
        let workflow = Workflow::new(
            id.clone(),
            name.clone(),
            project_id.clone(),
            description.clone(),
            paused.clone(),
            Some(created_at.clone()),
            Some(updated_at.clone()),
        )
        .expect("workflow should be created properly");
        let dto = WorkflowDto::from(workflow);
        assert_eq!(dto.id, id.to_uuid());
        assert_eq!(dto.name, name.into_string());
        assert_eq!(dto.project_id, project_id.to_uuid());
        assert_eq!(dto.description, description.into_string());
        assert_eq!(dto.paused, paused.to_bool());
        assert_eq!(dto.created_at, Some(created_at));
        assert_eq!(dto.updated_at, Some(updated_at));
    }
}

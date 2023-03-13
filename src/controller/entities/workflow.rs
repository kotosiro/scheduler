use super::project::ProjectId;
use crate::impl_bool_property;
use crate::impl_string_property;
use crate::impl_uuid_property;
use anyhow::Result;
use getset::Getters;
use getset::Setters;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowId {
    value: Uuid,
}

impl_uuid_property!(WorkflowId);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct WorkflowName {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(WorkflowName);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct WorkflowDescription {
    #[validate(length(min = 0))]
    value: String,
}

impl_string_property!(WorkflowDescription);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowPaused {
    value: bool,
}

impl_bool_property!(WorkflowPaused);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, serde::Serialize)]
pub struct Workflow {
    #[getset(get = "pub")]
    id: WorkflowId,
    #[getset(get = "pub", set = "pub")]
    name: WorkflowName,
    #[getset(get = "pub", set = "pub")]
    project_id: ProjectId,
    #[getset(get = "pub", set = "pub")]
    description: WorkflowDescription,
    #[getset(get = "pub", set = "pub")]
    paused: WorkflowPaused,
}

impl Workflow {
    pub fn new(
        id: String,
        name: String,
        project_id: String,
        description: String,
        paused: bool,
    ) -> Result<Self> {
        Ok(Self {
            id: WorkflowId::try_from(id)?,
            name: WorkflowName::new(name)?,
            project_id: ProjectId::try_from(project_id)?,
            description: WorkflowDescription::new(description)?,
            paused: WorkflowPaused::new(paused),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_workflow_id() {
        assert!(matches!(
            WorkflowId::try_from(testutils::rand::uuid()),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_workflow_id() {
        assert!(matches!(
            WorkflowId::try_from(testutils::rand::string(255)),
            Err(_)
        ));
    }

    #[test]
    fn test_valid_workflow_name() {
        assert!(matches!(
            WorkflowName::new(testutils::rand::string(255)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_workflow_name() {
        assert!(matches!(WorkflowName::new(""), Err(_)));
    }

    #[test]
    fn test_valid_workflow_description() {
        assert!(matches!(
            WorkflowDescription::new(testutils::rand::string(255)),
            Ok(_)
        ));
        assert!(matches!(WorkflowDescription::new(""), Ok(_)));
    }
}

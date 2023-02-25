use super::workflow::WorkflowId;
use crate::impl_i32_property;
use crate::impl_string_property;
use crate::impl_uuid_property;
use anyhow::Result;
use chrono::NaiveDateTime;
use getset::Getters;
use getset::Setters;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobId {
    value: Uuid,
}

impl_uuid_property!(JobId);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct JobName {
    #[validate(length(min = 1))]
    value: String,
}

impl_string_property!(JobName);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct JobThreshold {
    #[validate(range(min = 0, max = 100))]
    value: i32,
}

impl_i32_property!(JobThreshold);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct JobImage {
    #[validate(length(min = 0))]
    value: String,
}

impl_string_property!(JobImage);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct JobArg {
    #[validate(length(min = 0))]
    value: String,
}

impl_string_property!(JobArg);

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct JobEnv {
    #[validate(length(min = 0))]
    value: String,
}

impl_string_property!(JobEnv);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct Job {
    #[getset(get = "pub")]
    id: JobId,
    #[getset(get = "pub", set = "pub")]
    name: JobName,
    #[getset(get = "pub", set = "pub")]
    workflow_id: WorkflowId,
    #[getset(get = "pub", set = "pub")]
    threshold: JobThreshold,
    #[getset(get = "pub", set = "pub")]
    image: JobImage,
    #[getset(get = "pub", set = "pub")]
    args: Vec<JobArg>,
    #[getset(get = "pub", set = "pub")]
    envs: Vec<JobEnv>,
    #[getset(get = "pub", set = "pub")]
    created_at: Option<NaiveDateTime>,
    #[getset(get = "pub", set = "pub")]
    updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobDestructor {
    pub id: JobId,
    pub name: JobName,
    pub workflow_id: WorkflowId,
    pub threshold: JobThreshold,
    pub image: JobImage,
    pub args: Vec<JobArg>,
    pub envs: Vec<JobEnv>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Job {
    pub fn new(
        id: JobId,
        name: JobName,
        workflow_id: WorkflowId,
        threshold: JobThreshold,
        image: JobImage,
        args: Vec<JobArg>,
        envs: Vec<JobEnv>,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Result<Self> {
        Ok(Self {
            id,
            name,
            workflow_id,
            threshold,
            image,
            args,
            envs,
            created_at,
            updated_at,
        })
    }

    pub fn destruct(mut self) -> JobDestructor {
        JobDestructor {
            id: self.id,
            name: self.name,
            workflow_id: self.workflow_id,
            threshold: self.threshold,
            image: self.image,
            args: self.args,
            envs: self.envs,
            created_at: self.created_at.take(),
            updated_at: self.updated_at.take(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_job_id() {
        assert!(matches!(JobId::try_from(testutils::rand::uuid()), Ok(_)));
    }

    #[test]
    fn test_invalid_job_id() {
        assert!(matches!(
            JobId::try_from(testutils::rand::string(255)),
            Err(_)
        ));
    }

    #[test]
    fn test_valid_job_name() {
        assert!(matches!(JobName::new(testutils::rand::string(255)), Ok(_)));
    }

    #[test]
    fn test_invalid_job_name() {
        assert!(matches!(JobName::new(""), Err(_)));
    }

    #[test]
    fn test_valid_job_threshold() {
        assert!(matches!(
            JobThreshold::new(testutils::rand::i32(0, 100)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_job_threshold() {
        assert!(matches!(
            JobThreshold::new(testutils::rand::i32(-1000, -1)),
            Err(_)
        ));
        assert!(matches!(
            JobThreshold::new(testutils::rand::i32(101, 1000)),
            Err(_)
        ));
    }

    #[test]
    fn test_valid_job_image() {
        assert!(matches!(JobImage::new(testutils::rand::string(255)), Ok(_)));
        assert!(matches!(JobImage::new(""), Ok(_)));
    }

    #[test]
    fn test_valid_job_arg() {
        assert!(matches!(JobArg::new(testutils::rand::string(255)), Ok(_)));
        assert!(matches!(JobArg::new(""), Ok(_)));
    }

    #[test]
    fn test_valid_job_env() {
        assert!(matches!(JobEnv::new(testutils::rand::string(255)), Ok(_)));
        assert!(matches!(JobEnv::new(""), Ok(_)));
    }
}

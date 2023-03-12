use super::workflow::WorkflowId;
use crate::impl_i32_property;
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
}

impl Job {
    pub fn new(
        id: String,
        name: String,
        workflow_id: String,
        threshold: i32,
        image: String,
        args: Vec<String>,
        envs: Vec<String>,
    ) -> Result<Self> {
        Ok(Self {
            id: JobId::try_from(id)?,
            name: JobName::new(name)?,
            workflow_id: WorkflowId::try_from(workflow_id)?,
            threshold: JobThreshold::new(threshold)?,
            image: JobImage::new(image)?,
            args: args.into_iter().map(|a| JobArg::new(a)).flatten().collect(),
            envs: envs.into_iter().map(|e| JobEnv::new(e)).flatten().collect(),
        })
    }
}

impl TryFrom<Json> for Job {
    type Error = anyhow::Error;

    fn try_from(json: Json) -> std::result::Result<Self, Self::Error> {
        let id = json!(Uuid::new_v4().to_string());
        let id = match &json["id"] {
            Json::Null => &id,
            value => value,
        };
        let args = Vec::new();
        let args = match &json["args"] {
            Json::Null => &args,
            value => value.as_array().unwrap_or(&args),
        };
        let envs = Vec::new();
        let envs = match &json["envs"] {
            Json::Null => &envs,
            value => value.as_array().unwrap_or(&envs),
        };
        Ok(Self {
            id: JobId::try_from(id)?,
            name: JobName::try_from(&json["name"])?,
            workflow_id: WorkflowId::try_from(&json["workflow_id"])?,
            threshold: JobThreshold::try_from(&json["threshold"])?,
            image: JobImage::try_from(&json["image"])?,
            args: args
                .into_iter()
                .map(|a| JobArg::try_from(a))
                .flatten()
                .collect(),
            envs: envs
                .into_iter()
                .map(|e| JobEnv::try_from(e))
                .flatten()
                .collect(),
        })
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

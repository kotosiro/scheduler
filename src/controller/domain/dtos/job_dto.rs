use crate::controller::domain::entities::job::Job;
use crate::controller::domain::entities::job::JobDestructor;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct JobDto {
    pub id: Uuid,
    pub name: String,
    pub workflow_id: Uuid,
    pub threshold: i32,
    pub image: String,
    pub args: Vec<String>,
    pub envs: Vec<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<Job> for JobDto {
    fn from(job: Job) -> Self {
        let JobDestructor {
            id,
            name,
            workflow_id,
            threshold,
            image,
            args,
            envs,
            mut created_at,
            mut updated_at,
        } = job.destruct();
        Self {
            id: id.to_uuid(),
            name: name.into_string(),
            workflow_id: workflow_id.to_uuid(),
            threshold: threshold.to_i32(),
            image: image.into_string(),
            args: args.into_iter().map(|arg| arg.into_string()).collect(),
            envs: envs.into_iter().map(|env| env.into_string()).collect(),
            created_at: created_at.take(),
            updated_at: updated_at.take(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::domain::entities::job::Job;
    use crate::controller::domain::entities::job::JobArg;
    use crate::controller::domain::entities::job::JobEnv;
    use crate::controller::domain::entities::job::JobId;
    use crate::controller::domain::entities::job::JobImage;
    use crate::controller::domain::entities::job::JobName;
    use crate::controller::domain::entities::job::JobThreshold;
    use crate::controller::domain::entities::workflow::WorkflowId;

    #[test]
    fn test_from() {
        let id =
            JobId::try_from(testutils::rand::uuid()).expect("job id should be parsed properly");
        let name =
            JobName::new(testutils::rand::string(10)).expect("job name should be parsed properly");
        let workflow_id = WorkflowId::try_from(testutils::rand::uuid())
            .expect("workflow id should be parsed properly");
        let threshold = JobThreshold::new(testutils::rand::i32(0, 100))
            .expect("job threshold should be parsed properly");
        let image = JobImage::new(testutils::rand::string(10))
            .expect("job image should be parsed properly");
        let mut args = Vec::new();
        let len_args = testutils::rand::i32(0, 10);
        for _ in 0..len_args {
            let arg = JobArg::new(testutils::rand::string(10))
                .expect("job arg should be parsed properly");
            args.push(arg)
        }
        let mut envs = Vec::new();
        let len_envs = testutils::rand::i32(0, 10);
        for _ in 0..len_envs {
            let env = JobEnv::new(testutils::rand::string(10))
                .expect("job env should be parsed properly");
            envs.push(env)
        }
        let created_at = testutils::rand::now();
        let updated_at = testutils::rand::now();
        let job = Job::new(
            id.clone(),
            name.clone(),
            workflow_id.clone(),
            threshold.clone(),
            image.clone(),
            args.to_vec(),
            envs.to_vec(),
            Some(created_at.clone()),
            Some(updated_at.clone()),
        )
        .expect("job should be created properly");
        let dto = JobDto::from(job);
        assert_eq!(dto.id, id.to_uuid());
        assert_eq!(dto.name, name.into_string());
        assert_eq!(dto.workflow_id, workflow_id.to_uuid());
        assert_eq!(dto.threshold, threshold.to_i32());
        assert_eq!(dto.image, image.into_string());
        assert_eq!(
            dto.args,
            args.into_iter()
                .map(|arg| arg.into_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            dto.envs,
            envs.into_iter()
                .map(|env| env.into_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(dto.created_at, Some(created_at));
        assert_eq!(dto.updated_at, Some(updated_at));
    }
}

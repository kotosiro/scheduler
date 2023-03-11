use crate::controller::domain::entities::job::Job;
use crate::controller::domain::entities::job::JobId;
use crate::middlewares::postgres::PgAcquire;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::Utc;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct JobRow {
    pub id: Uuid,
    pub name: String,
    pub workflow_id: Uuid,
    pub threshold: i32,
    pub image: String,
    pub args: Vec<String>,
    pub envs: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait JobRepository: Send + Sync + 'static {
    async fn create(
        &self,
        job: &Job,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult>;

    async fn delete(
        &self,
        id: &JobId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult>;

    async fn get_by_id(
        &self,
        id: &JobId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<JobRow>>;
}

pub struct PgJobRepository;

#[async_trait]
impl JobRepository for PgJobRepository {
    async fn create(
        &self,
        job: &Job,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "INSERT INTO job (
                 id,
                 name,
                 workflow_id,
                 threshold,
                 image,
                 args,
                 envs
             ) VALUES ($1, $2, $3, $4, $5, $6, $7)
             ON CONFLICT(name, workflow_id)
             DO UPDATE
             SET threshold = $4,
                 image = $5,
                 args = $6,
                 envs = $7",
        )
        .bind(job.id().as_uuid())
        .bind(job.name())
        .bind(job.workflow_id().as_uuid())
        .bind(job.threshold().as_i32())
        .bind(job.image())
        .bind(job.args())
        .bind(job.envs())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [job]"#,
            job.id().as_uuid()
        ))
    }

    async fn delete(
        &self,
        id: &JobId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "DELETE FROM job
             WHERE id = $1",
        )
        .bind(id.as_uuid())
        .execute(&mut *conn)
        .await
        .context(format!(r#"failed to delete "{}" from [job]"#, id.as_uuid()))
    }

    async fn get_by_id(
        &self,
        id: &JobId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<JobRow>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<JobRow> = sqlx::query_as::<_, JobRow>(
            "SELECT
                 id,
                 name,
                 workflow_id,
                 threshold,
                 image,
                 args,
                 envs,
                 created_at,
                 updated_at
             FROM job
             WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&mut *conn)
        .await
        .context(format!(r#"failed to select "{}" from [job]"#, id.as_uuid()))?;
        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::domain::entities::project::Project;
    use crate::controller::domain::entities::project::ProjectId;
    use crate::controller::domain::entities::workflow::Workflow;
    use crate::controller::domain::entities::workflow::WorkflowId;
    use crate::controller::domain::repositories::project::PgProjectRepository;
    use crate::controller::domain::repositories::project::ProjectRepository;
    use crate::controller::domain::repositories::workflow::PgWorkflowRepository;
    use crate::controller::domain::repositories::workflow::WorkflowRepository;
    use anyhow::Context;
    use anyhow::Result;
    use sqlx::PgConnection;
    use sqlx::PgPool;

    async fn create_project(tx: &mut PgConnection) -> Result<Project> {
        let repo = PgProjectRepository;
        let project = Project::new(
            testutils::rand::uuid(),
            testutils::rand::string(10),
            testutils::rand::string(10),
            None,
        )
        .context("failed to create project")?;
        repo.create(&project, tx)
            .await
            .context("failed to insert project")?;
        Ok(project)
    }

    async fn create_workflow(project_id: &ProjectId, tx: &mut PgConnection) -> Result<Workflow> {
        let repo = PgWorkflowRepository;
        let workflow = Workflow::new(
            testutils::rand::uuid(),
            testutils::rand::string(10),
            project_id.as_uuid().to_string(),
            testutils::rand::string(10),
            testutils::rand::bool(),
        )
        .context("failed to create workflow")?;
        repo.create(&workflow, tx)
            .await
            .context("failed to insert workflow")?;
        Ok(workflow)
    }

    async fn create_job(workflow_id: &WorkflowId, tx: &mut PgConnection) -> Result<Job> {
        let repo = PgJobRepository;
        let num_args = testutils::rand::usize(10);
        let mut args = Vec::new();
        for _ in 0..num_args {
            args.push(testutils::rand::string(10));
        }
        let num_envs = testutils::rand::usize(10);
        let mut envs = Vec::new();
        for _ in 0..num_envs {
            envs.push(testutils::rand::string(10));
        }
        let job = Job::new(
            testutils::rand::uuid(),
            testutils::rand::string(10),
            workflow_id.as_uuid().to_string(),
            testutils::rand::i32(0, 10),
            testutils::rand::string(10),
            args,
            envs,
        )
        .context("failed to create job")?;
        repo.create(&job, tx)
            .await
            .context("failed to insert job")?;
        Ok(job)
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_get_by_id(pool: PgPool) -> Result<()> {
        let repo = PgJobRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let project = create_project(&mut tx)
            .await
            .expect("new project should be created");
        let workflow = create_workflow(&project.id(), &mut tx)
            .await
            .expect("new workflow should be created");
        let job = create_job(&workflow.id(), &mut tx)
            .await
            .expect("new job should be created");
        let fetched = repo
            .get_by_id(&job.id(), &mut tx)
            .await
            .expect("inserted job should be found");
        if let Some(fetched) = fetched {
            assert_eq!(&fetched.id, job.id().as_uuid());
            assert_eq!(&fetched.name, job.name().as_str());
            assert_eq!(&fetched.workflow_id, job.workflow_id().as_uuid());
            assert_eq!(&fetched.threshold, job.threshold().as_i32());
            assert_eq!(&fetched.image, job.image().as_str());
            assert_eq!(
                fetched.args,
                job.args()
                    .into_iter()
                    .map(|a| a.as_str())
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                fetched.envs,
                job.envs()
                    .into_iter()
                    .map(|e| e.as_str())
                    .collect::<Vec<_>>()
            );
        } else {
            panic!("inserted job should be found");
        }
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }
}

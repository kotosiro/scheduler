use crate::controller::domain::dtos::run::Run as RunRow;
use crate::controller::domain::entities::run::Run;
use crate::controller::domain::entities::run::RunId;
use crate::middlewares::postgres::PgAcquire;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;

#[async_trait]
pub trait RunRepository: Send + Sync + 'static {
    async fn create(
        &self,
        run: &Run,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult>;

    async fn delete(
        &self,
        id: &RunId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult>;

    async fn get_by_id(
        &self,
        id: &RunId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<RunRow>>;
}

pub struct PgRunRepository;

#[async_trait]
impl RunRepository for PgRunRepository {
    async fn create(
        &self,
        run: &Run,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "INSERT INTO run (
                 id,
                 state,
                 priority,
                 job_id,
                 triggered_at,
                 started_at,
                 finished_at
             ) VALUES ($1, $2, $3, $4, $5, NULL, NULL)",
        )
        .bind(run.id().as_uuid())
        .bind(run.state())
        .bind(run.priority())
        .bind(run.job_id().as_uuid())
        .bind(run.triggered_at())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [run]"#,
            run.id().as_uuid()
        ))
    }

    async fn delete(
        &self,
        id: &RunId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "DELETE FROM run
             WHERE id = $1",
        )
        .bind(id.as_uuid())
        .execute(&mut *conn)
        .await
        .context(format!(r#"failed to delete "{}" from [run]"#, id.as_uuid()))
    }

    async fn get_by_id(
        &self,
        id: &RunId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<RunRow>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<RunRow> = sqlx::query_as::<_, RunRow>(
            "SELECT
                 id,
                 state,
                 priority,
                 job_id,
                 triggered_at,
                 started_at,
                 finished_at,
                 created_at,
                 updated_at
             FROM run
             WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&mut *conn)
        .await
        .context(format!(r#"failed to select "{}" from [run]"#, id.as_uuid()))?;
        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::domain::entities::job::Job;
    use crate::controller::domain::entities::job::JobId;
    use crate::controller::domain::entities::project::Project;
    use crate::controller::domain::entities::project::ProjectId;
    use crate::controller::domain::entities::run::RunPriority;
    use crate::controller::domain::entities::token::TokenState;
    use crate::controller::domain::entities::workflow::Workflow;
    use crate::controller::domain::entities::workflow::WorkflowId;
    use crate::controller::domain::repositories::job::JobRepository;
    use crate::controller::domain::repositories::job::PgJobRepository;
    use crate::controller::domain::repositories::project::PgProjectRepository;
    use crate::controller::domain::repositories::project::ProjectRepository;
    use crate::controller::domain::repositories::workflow::PgWorkflowRepository;
    use crate::controller::domain::repositories::workflow::WorkflowRepository;
    use anyhow::Context;
    use anyhow::Result;
    use chrono::Utc;
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

    async fn create_run(job_id: &JobId, tx: &mut PgConnection) -> Result<Run> {
        let repo = PgRunRepository;
        let states = vec![
            TokenState::Waiting,
            TokenState::Active,
            TokenState::Running,
            TokenState::Success,
            TokenState::Failure,
            TokenState::Error,
        ];
        let state = testutils::rand::choice(&states);
        let priorities = vec![
            RunPriority::BackFill,
            RunPriority::Low,
            RunPriority::Normal,
            RunPriority::High,
        ];
        let priority = testutils::rand::choice(&priorities);
        let now = Utc::now();
        let run = Run::new(
            testutils::rand::uuid(),
            *state,
            *priority,
            job_id.as_uuid().to_string(),
            now,
        )
        .context("failed to create run")?;
        repo.create(&run, tx)
            .await
            .context("failed to insert run")?;
        Ok(run)
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_get_by_id(pool: PgPool) -> Result<()> {
        let repo = PgRunRepository;
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
        let run = create_run(&job.id(), &mut tx)
            .await
            .expect("new run should be created");
        let fetched = repo
            .get_by_id(&run.id(), &mut tx)
            .await
            .expect("inserted run should be found");
        if let Some(fetched) = fetched {
            assert_eq!(&fetched.id, run.id().as_uuid());
            assert_eq!(&fetched.state, run.state().as_ref());
            assert_eq!(&fetched.priority, run.priority().as_ref());
            assert!(&fetched.started_at.is_none());
            assert!(&fetched.finished_at.is_none());
        } else {
            panic!("inserted job should be found");
        }
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }
}

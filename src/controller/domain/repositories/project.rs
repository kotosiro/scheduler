use crate::controller::domain::dtos::project::Project as ProjectRow;
use crate::controller::domain::dtos::project::ProjectConfig as ProjectConfigRow;
use crate::controller::domain::dtos::project::ProjectSummary as ProjectSummaryRow;
use crate::controller::domain::dtos::workflow::WorkflowSummary as WorkflowSummaryRow;
use crate::controller::domain::entities::project::Project;
use crate::controller::domain::entities::project::ProjectId;
use crate::controller::domain::entities::project::ProjectName;
use crate::controller::domain::entities::workflow::WorkflowName;
use crate::middlewares::postgres::PgAcquire;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;

#[async_trait]
pub trait ProjectRepository: Send + Sync + 'static {
    async fn create(
        &self,
        project: &Project,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult>;

    async fn delete(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult>;

    async fn list(
        &self,
        limit: Option<i64>,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Vec<ProjectRow>>;

    async fn get_by_id(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<ProjectRow>>;

    async fn get_by_name(
        &self,
        name: &ProjectName,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<ProjectRow>>;

    async fn get_summary_by_id(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<ProjectSummaryRow>>;

    async fn get_config_by_id(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<ProjectConfigRow>>;

    async fn list_workflows_by_id(
        &self,
        id: &ProjectId,
        name: Option<&WorkflowName>,
        after: Option<&WorkflowName>,
        limit: Option<i64>,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Vec<WorkflowSummaryRow>>;
}

pub struct PgProjectRepository;

#[async_trait]
impl ProjectRepository for PgProjectRepository {
    async fn create(
        &self,
        project: &Project,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "INSERT INTO project (
                 id,
                 name,
                 description,
                 config
             ) VALUES ($1, $2, $3, $4)
             ON CONFLICT(id)
             DO UPDATE
             SET name = $2,
                 description = $3,
                 config = COALESCE($4, project.config)",
        )
        .bind(project.id().as_uuid())
        .bind(project.name().as_str())
        .bind(project.description().as_str())
        .bind(project.config().as_ref().map(|config| config.as_json()))
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [project]"#,
            project.id().as_uuid()
        ))
    }

    async fn delete(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "DELETE FROM project
             WHERE id = $1",
        )
        .bind(id.as_uuid())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to delete "{}" from [project]"#,
            id.as_uuid()
        ))
    }

    async fn list(
        &self,
        limit: Option<i64>,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Vec<ProjectRow>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let rows: Vec<ProjectRow> = sqlx::query_as::<_, ProjectRow>(
            "SELECT
                 id,
                 name,
                 description,
                 COALESCE(config, '{}'::jsonb) AS config,
                 created_at,
                 updated_at
             FROM project
             ORDER BY name
             LIMIT $1",
        )
        .bind(limit.unwrap_or(100))
        .fetch_all(&mut *conn)
        .await
        .context(format!(
            "failed to list {} project(s) from [project]",
            limit.unwrap_or(100)
        ))?;
        Ok(rows)
    }

    async fn get_by_id(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<ProjectRow>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<ProjectRow> = sqlx::query_as::<_, ProjectRow>(
            "SELECT
                 id,
                 name,
                 description,
                 COALESCE(config, '{}'::jsonb) AS config,
                 created_at,
                 updated_at
             FROM project
             WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [project]"#,
            id.as_uuid()
        ))?;
        Ok(row)
    }

    async fn get_by_name(
        &self,
        name: &ProjectName,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<ProjectRow>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<ProjectRow> = sqlx::query_as::<_, ProjectRow>(
            "SELECT
                 id,
                 name,
                 description,
                 COALESCE(config, '{}'::jsonb) AS config,
                 created_at,
                 updated_at
             FROM project
             WHERE name = $1",
        )
        .bind(name.as_str())
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [project]"#,
            name.as_str()
        ))?;
        Ok(row)
    }

    async fn get_summary_by_id(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<ProjectSummaryRow>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<ProjectSummaryRow> = sqlx::query_as::<_, ProjectSummaryRow>(
            "WITH these_jobs AS (
                 SELECT
                     job.id AS id,
                     run.state AS state
                 FROM workflow
                 JOIN job ON job.workflow_id = workflow.id
                 JOIN run ON run.job_id = job.id
                 WHERE workflow.project_id = $1
                 AND (finished_at IS NULL OR CURRENT_TIMESTAMP - finished_at < INTERVAL '1 hour')
             )
             SELECT
                 id,
                 name,
                 description,
                 (
                     SELECT COUNT(1)
                     FROM workflow
                     WHERE workflow.project_id = $1
                 ) AS workflows,
                 (
                     SELECT COUNT(1)
                     FROM these_jobs
                     WHERE (these_jobs.state = 'running')
                 ) AS running_jobs,
                 (
                     SELECT COUNT(1)
                     FROM these_jobs
                     WHERE (these_jobs.state = 'waiting' OR these_jobs.state = 'active')
                 ) AS waiting_jobs,
                 (
                     SELECT COUNT(1)
                     FROM these_jobs
                     WHERE these_jobs.state = 'failure'
                 ) AS fails_last_hour,
                 (
                     SELECT COUNT(1)
                     FROM these_jobs
                     WHERE these_jobs.state = 'success'
                 ) AS successes_last_hour,
                 (
                     SELECT COUNT(1)
                     FROM these_jobs
                     WHERE these_jobs.state = 'error'
                 ) AS errors_last_hour
             FROM project
             WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to summarize "{}" from [project]"#,
            id.as_uuid()
        ))?;
        Ok(row)
    }

    async fn get_config_by_id(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<ProjectConfigRow>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<ProjectConfigRow> = sqlx::query_as::<_, ProjectConfigRow>(
            "SELECT COALESCE(config, '{}'::jsonb) AS config
             FROM project
             WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select config of "{}" from [project]"#,
            id.as_uuid()
        ))?;
        Ok(row)
    }

    async fn list_workflows_by_id(
        &self,
        id: &ProjectId,
        name: Option<&WorkflowName>,
        after: Option<&WorkflowName>,
        limit: Option<i64>,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Vec<WorkflowSummaryRow>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let rows: Vec<WorkflowSummaryRow> = sqlx::query_as::<_, WorkflowSummaryRow>(
            "WITH these_runs AS (
                 SELECT
                     job.workflow_id AS workflow_id,
                     run.state AS state
                 FROM workflow
                 JOIN job ON workflow.id = job.workflow_id
                 LEFT OUTER JOIN run ON run.job_id = job.id
                 WHERE workflow.project_id = $1 AND (
                     run.finished_at IS NULL
                     OR CURRENT_TIMESTAMP - run.finished_at < INTERVAL '1 hour'
                 )
             ),
             summaries AS (
                 SELECT
                     workflow_id,
                     SUM(CASE WHEN state = 'success' THEN 1 ELSE 0 END) AS success,
                     SUM(CASE WHEN state = 'running' THEN 1 ELSE 0 END) AS running,
                     SUM(CASE WHEN state = 'failure' THEN 1 ELSE 0 END) AS failure,
                     SUM(CASE
                             WHEN state = 'active' OR state = 'waiting' THEN 1
                             ELSE 0
                         END) AS waiting,
                     SUM(CASE WHEN state = 'error' THEN 1 ELSE 0 END) as error
                 FROM these_runs
                 GROUP BY workflow_id
             )
             SELECT
                 id,
                 name,
                 description,
                 paused,
                 COALESCE(success, 0) AS success,
                 COALESCE(running, 0) AS running,
                 COALESCE(failure, 0) AS failure,
                 COALESCE(waiting, 0) AS waiting,
                 COALESCE(error,   0) AS error
             FROM workflow
             LEFT OUTER JOIN summaries ON workflow.id = summaries.workflow_id
             WHERE
                 project_id = $1
                 AND ($3 IS NULL OR name = $2)
                 AND ($2 IS NULL OR name > $3)
             ORDER BY name
             LIMIT $4",
        )
        .bind(id.as_uuid())
        .bind(name.map(|n| n.as_str()))
        .bind(after.map(|n| n.as_str()))
        .bind(limit.unwrap_or(100))
        .fetch_all(&mut *conn)
        .await
        .context(format!(
            r#"failed to list {} workflow summary(ies) of "{}" from [project]"#,
            limit.unwrap_or(100),
            id.as_uuid()
        ))?;
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::domain::entities::job::Job;
    use crate::controller::domain::entities::job::JobId;
    use crate::controller::domain::entities::run::Run;
    use crate::controller::domain::entities::run::RunPriority;
    use crate::controller::domain::entities::token::TokenState;
    use crate::controller::domain::entities::workflow::Workflow;
    use crate::controller::domain::entities::workflow::WorkflowId;
    use crate::controller::domain::repositories::job::JobRepository;
    use crate::controller::domain::repositories::job::PgJobRepository;
    use crate::controller::domain::repositories::run::PgRunRepository;
    use crate::controller::domain::repositories::run::RunRepository;
    use crate::controller::domain::repositories::workflow::PgWorkflowRepository;
    use crate::controller::domain::repositories::workflow::WorkflowRepository;
    use anyhow::Context;
    use anyhow::Result;
    use chrono::Utc;
    use serde_json::Value as Json;
    use sqlx::PgConnection;
    use sqlx::PgPool;
    use std::cmp::min;

    async fn create_project(config: Option<Json>, tx: &mut PgConnection) -> Result<Project> {
        let repo = PgProjectRepository;
        let project = Project::new(
            testutils::rand::uuid(),
            testutils::rand::string(10),
            testutils::rand::string(10),
            config,
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
            testutils::rand::i64(0, 10),
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
        let repo = PgProjectRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let project = create_project(None, &mut tx)
            .await
            .expect("new project should be created");
        let fetched = repo
            .get_by_id(&project.id(), &mut tx)
            .await
            .expect("inserted project should be found");
        if let Some(fetched) = fetched {
            assert_eq!(&fetched.id, project.id().as_uuid());
            assert_eq!(&fetched.name, project.name().as_str());
            assert_eq!(&fetched.description, project.description().as_str());
            assert!(fetched.config.is_some());
        } else {
            panic!("inserted project should be found");
        }
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_get_by_name(pool: PgPool) -> Result<()> {
        let repo = PgProjectRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let project = create_project(None, &mut tx)
            .await
            .expect("new project should be created");
        let fetched = repo
            .get_by_name(&project.name(), &mut tx)
            .await
            .expect("inserted project should be found");
        if let Some(fetched) = fetched {
            assert_eq!(&fetched.id, project.id().as_uuid());
            assert_eq!(&fetched.name, project.name().as_str());
            assert_eq!(&fetched.description, project.description().as_str());
            assert!(fetched.config.is_some());
        } else {
            panic!("inserted project should be found");
        }
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_list_with_default_limit(pool: PgPool) -> Result<()> {
        let repo = PgProjectRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let records = testutils::rand::i64(0, 200);
        for _ in 0..records {
            create_project(None, &mut tx)
                .await
                .expect("new project should be created");
        }
        let fetched = repo
            .list(None, &mut tx)
            .await
            .expect("inserted project should be listed");
        assert_eq!(min(records, 100) as usize, fetched.len());
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_list_with_specified_limit(pool: PgPool) -> Result<()> {
        let repo = PgProjectRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let records = testutils::rand::i64(0, 200);
        for _ in 0..records {
            create_project(None, &mut tx)
                .await
                .expect("new project should be created");
        }
        let limit = testutils::rand::i64(0, 200);
        let fetched = repo
            .list(Some(limit.into()), &mut tx)
            .await
            .expect("inserted project should be listed");
        assert_eq!(min(records, limit) as usize, fetched.len());
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_get_summary(pool: PgPool) -> Result<()> {
        let repo = PgProjectRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let project = create_project(None, &mut tx)
            .await
            .expect("new project should be created");
        let num_workflows = testutils::rand::usize(10);
        let num_jobs = testutils::rand::usize(100);
        for _ in 0..num_workflows {
            let workflow = create_workflow(&project.id(), &mut tx)
                .await
                .expect("new workflow should be created");
            for _ in 0..num_jobs {
                let job = create_job(&workflow.id(), &mut tx)
                    .await
                    .expect("new job should be created");
                let _ = create_run(&job.id(), &mut tx)
                    .await
                    .expect("new run should be created");
            }
        }
        let fetched = repo
            .get_summary_by_id(&project.id(), &mut tx)
            .await
            .expect("inserted project should be found");
        if let Some(fetched) = fetched {
            assert_eq!(
                (&num_workflows * &num_jobs) as i64,
                &fetched.running_jobs
                    + &fetched.waiting_jobs
                    + &fetched.fails_last_hour
                    + &fetched.successes_last_hour
                    + &fetched.errors_last_hour
            );
        } else {
            panic!("inserted job should be found");
        }
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_get_config_by_id(pool: PgPool) -> Result<()> {
        let repo = PgProjectRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let config = format!(
            r#"{{
                    "id": "{}",
                    "division": {},
                    "members": ["{}", "{}"]
               }}"#,
            testutils::rand::uuid(),
            testutils::rand::i64(0, 1000),
            testutils::rand::string(10),
            testutils::rand::string(10),
        );
        let config: Json = serde_json::from_str(&config).context("failed to create config")?;
        let project = create_project(Some(config.clone()), &mut tx)
            .await
            .expect("new project should be created");
        let fetched = repo
            .get_config_by_id(&project.id(), &mut tx)
            .await
            .expect("inserted project should be found");
        if let Some(fetched) = fetched {
            let ProjectConfigRow(fetched) = fetched;
            assert_eq!(&fetched, &config);
        } else {
            panic!("inserted project should have config");
        }
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_list_workflows(pool: PgPool) -> Result<()> {
        let repo = PgProjectRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let project = create_project(None, &mut tx)
            .await
            .expect("new project should be created");
        let num_workflows = testutils::rand::usize(10);
        let num_jobs = testutils::rand::usize(100);
        for _ in 0..num_workflows {
            let workflow = create_workflow(&project.id(), &mut tx)
                .await
                .expect("new workflow should be created");
            for _ in 0..num_jobs {
                let job = create_job(&workflow.id(), &mut tx)
                    .await
                    .expect("new job should be created");
                let _ = create_run(&job.id(), &mut tx)
                    .await
                    .expect("new run should be created");
            }
        }
        let fetched = repo
            .list_workflows_by_id(&project.id(), None, None, None, &mut tx)
            .await
            .expect("inserted project should be found");
        let len = fetched.len();
        let sum = fetched
            .into_iter()
            .map(|s| s.success + s.running + s.failure + s.waiting + s.error)
            .sum::<i64>();
        assert_eq!(num_workflows, len);
        assert_eq!((num_workflows * num_jobs) as i64, sum);
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }
}

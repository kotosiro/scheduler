use crate::controller::domain::dtos::workflow::Workflow as WorkflowRow;
use crate::controller::domain::entities::workflow::Workflow;
use crate::controller::domain::entities::workflow::WorkflowId;
use crate::infra::postgres::PgAcquire;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;

#[async_trait]
pub trait WorkflowRepository: Send + Sync + 'static {
    async fn create(
        &self,
        workflow: &Workflow,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult>;

    async fn delete(
        &self,
        id: &WorkflowId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult>;

    async fn find_by_id(
        &self,
        id: &WorkflowId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<WorkflowRow>>;
}

pub struct PgWorkflowRepository;

#[async_trait]
impl WorkflowRepository for PgWorkflowRepository {
    async fn create(
        &self,
        workflow: &Workflow,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "INSERT INTO workflow (
                 id,
                 name,
                 project_id,
                 description,
                 paused
             ) VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT(id)
             DO UPDATE
             SET name = $2,
                 project_id = $3,
                 description = $4,
                 paused = $5",
        )
        .bind(workflow.id().as_uuid())
        .bind(workflow.name().as_str())
        .bind(workflow.project_id().as_uuid())
        .bind(workflow.description().as_str())
        .bind(workflow.paused().as_bool())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to upsert "{}" into [workflow]"#,
            workflow.id().as_uuid()
        ))
    }

    async fn delete(
        &self,
        id: &WorkflowId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        sqlx::query(
            "DELETE FROM workflow
             WHERE id = $1",
        )
        .bind(id.as_uuid())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to delete "{}" from [workflow]"#,
            id.as_uuid()
        ))
    }

    async fn find_by_id(
        &self,
        id: &WorkflowId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<WorkflowRow>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<WorkflowRow> = sqlx::query_as::<_, WorkflowRow>(
            "SELECT
                 id,
                 name,
                 project_id,
                 description,
                 paused,
                 created_at,
                 updated_at
             FROM workflow
             WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [workflow]"#,
            id.as_uuid()
        ))?;
        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::domain::entities::project::Project;
    use crate::controller::domain::entities::project::ProjectId;
    use crate::controller::domain::repositories::project::PgProjectRepository;
    use crate::controller::domain::repositories::project::ProjectRepository;
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

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_find_by_id(pool: PgPool) -> Result<()> {
        let repo = PgWorkflowRepository;
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
        let fetched = repo
            .find_by_id(&workflow.id(), &mut tx)
            .await
            .expect("inserted workflow should be found");
        if let Some(fetched) = fetched {
            assert_eq!(&fetched.id, workflow.id().as_uuid());
            assert_eq!(&fetched.name, workflow.name().as_str());
            assert_eq!(&fetched.project_id, workflow.project_id().as_uuid());
            assert_eq!(&fetched.description, workflow.description().as_str());
            assert_eq!(&fetched.paused, workflow.paused().as_bool());
        } else {
            panic!("inserted workflow should be found");
        }
        tx.rollback()
            .await
            .expect("rollback should be done properly");
        Ok(())
    }
}

use crate::controller::domain::entities::project::Project;
use crate::controller::domain::entities::project::ProjectConfig;
use crate::controller::domain::entities::project::ProjectDescription;
use crate::controller::domain::entities::project::ProjectId;
use crate::controller::domain::entities::project::ProjectName;
use crate::infra::postgres::PgAcquire;
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde_json::Value as Json;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
struct ProjectRow {
    id: Uuid,
    name: String,
    description: String,
    config: Option<Json>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[async_trait]
pub trait ProjectRepository: Send + Sync + 'static {
    async fn create(
        &self,
        project: &Project,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<()>;

    async fn delete(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<()>;

    async fn find_by_id(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<Project>>;
}

pub struct PgProjectRepository;

#[async_trait]
impl ProjectRepository for PgProjectRepository {
    async fn create(
        &self,
        project: &Project,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<()> {
        let mut conn = executor.acquire().await.context("db connection failed")?;
        sqlx::query(
            "INSERT INTO project(id, name, description, config)
                     VALUES($1, $2, $3, $4)
                     ON CONFLICT(id)
                     DO UPDATE
                     SET name = $2,
                         description = $3,
                         config = COALESCE($4, project.config)",
        )
        .bind(project.id().to_uuid())
        .bind(project.name().as_str())
        .bind(project.description().as_str())
        .bind(project.config().as_ref().map(|config| config.to_json()))
        .execute(&mut *conn)
        .await
        .context("db query execution failed")?;
        Ok(())
    }

    async fn delete(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<()> {
        let mut conn = executor.acquire().await.context("db connection failed")?;
        sqlx::query(
            "DELETE FROM project
             WHERE id = $1",
        )
        .bind(id.to_uuid())
        .execute(&mut *conn)
        .await
        .context("db query execution failed")?;
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<Project>> {
        let mut conn = executor.acquire().await.context("db connection failed")?;
        let row: Option<ProjectRow> = sqlx::query_as::<_, ProjectRow>(
            "SELECT id, name, description, COALESCE(config, '{}'::jsonb) AS config, created_at, updated_at
             FROM project
             WHERE id = $1",
        )
        .bind(id.to_uuid())
        .fetch_optional(&mut *conn)
        .await.context("db query execution failed")?;

        let project = row.map(|mut row| {
            let id = ProjectId::new(row.id)?;
            let name = ProjectName::new(row.name)?;
            let description = ProjectDescription::new(row.description)?;
            let config = match row.config.take() {
                Some(json) => ProjectConfig::new(json).ok(),
                None => None,
            };
            Project::new(
                id,
                name,
                description,
                config,
                Some(row.created_at),
                Some(row.updated_at),
            )
        });
        let project = project.transpose()?;

        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use anyhow::Result;
    use sqlx::PgConnection;
    use sqlx::PgPool;

    async fn prepare_project(tx: &mut PgConnection) -> Result<Project> {
        let id = ProjectId::new(Uuid::new_v4()).context("cannot parse project id properly")?;
        let name = ProjectName::new(testutils::rand::string(10))
            .context("cannot parse project name properly")?;
        let description = ProjectDescription::new(testutils::rand::string(10))
            .context("cannot parse project description properly")?;
        let project = Project::new(id.clone(), name, description, None, None, None)
            .context("cannot create project properly")?;
        let repo = PgProjectRepository;
        repo.create(&project, tx)
            .await
            .context("cannot insert project properly")?;
        Ok(project)
    }

    #[sqlx::test]
    //    #[ignore] // Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_find_by_id(pool: PgPool) -> Result<()> {
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let repo = PgProjectRepository;

        let project = prepare_project(&mut tx)
            .await
            .expect("new project should be created");
        let fetched = repo
            .find_by_id(&project.id(), &mut tx)
            .await
            .expect("inserted project should be found");

        if let Some(fetched) = fetched {
            assert_eq!(fetched.id(), project.id());
            assert_eq!(fetched.name(), project.name());
            assert_eq!(fetched.description(), project.description());
            assert!(fetched.config().is_some());
            assert!(fetched.created_at().is_some());
            assert!(fetched.updated_at().is_some());
        } else {
            panic!("inserted project should be found");
        }

        tx.rollback()
            .await
            .expect("rollback should be done properly");

        Ok(())
    }
}

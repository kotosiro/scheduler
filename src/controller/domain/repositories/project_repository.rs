use crate::controller::domain::entities::project::Project;
use crate::controller::domain::entities::project::ProjectId;
use crate::infra::postgres::PgAcquire;
use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use serde_json::Value as Json;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct ProjectRow {
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
    ) -> Result<Option<ProjectRow>>;
}

pub struct PgProjectRepository;

#[async_trait]
impl ProjectRepository for PgProjectRepository {
    async fn create(
        &self,
        project: &Project,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<()> {
        let mut conn = executor.acquire().await?;
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
        .await?;
        Ok(())
    }

    async fn delete(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<()> {
        let mut conn = executor.acquire().await?;
        sqlx::query(
            "DELETE FROM project
             WHERE id = $1",
        )
        .bind(id.to_uuid())
        .execute(&mut *conn)
        .await?;
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<ProjectRow>> {
        let mut conn = executor.acquire().await?;
        let row: Option<ProjectRow> = sqlx::query_as(
            "SELECT id, name, description, COALESCE(config, '{}'::jsonb) AS config, created_at, updated_at
             FROM project
             WHERE id = $1",
        )
        .bind(id.to_uuid())
        //        .execute(&mut *conn)
        //        .bind(&id)
        //        .fetch_optional(&req.get_pool())
        .fetch_optional(&mut *conn)
        .await?;
        Ok(row)
    }
}

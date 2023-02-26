use crate::controller::domain::dtos::project::Project as ProjectRow;
use crate::controller::domain::entities::project::Project;
use crate::controller::domain::entities::project::ProjectId;
use crate::controller::domain::entities::project::ProjectName;
use crate::infra::postgres::PgAcquire;
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

    async fn find_by_id(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<ProjectRow>>;

    async fn find_by_name(
        &self,
        name: &ProjectName,
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
    ) -> Result<PgQueryResult> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
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
        .context(format!(
            r#"failed to upsert "{}" into [project]"#,
            project.id().to_uuid()
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
        .bind(id.to_uuid())
        .execute(&mut *conn)
        .await
        .context(format!(
            r#"failed to delete "{}" from [project]"#,
            id.to_uuid()
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
            "SELECT id, name, description, COALESCE(config, '{}'::jsonb) AS config, created_at, updated_at
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

    async fn find_by_id(
        &self,
        id: &ProjectId,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<ProjectRow>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<ProjectRow> =
        sqlx::query_as::<_, ProjectRow>(
            "SELECT id, name, description, COALESCE(config, '{}'::jsonb) AS config, created_at, updated_at
             FROM project
             WHERE id = $1",
        )
        .bind(id.to_uuid())
        .fetch_optional(&mut *conn)
        .await
        .context(format!(
            r#"failed to select "{}" from [project]"#,
            id.to_uuid()
        ))?;
        Ok(row)
    }

    async fn find_by_name(
        &self,
        name: &ProjectName,
        executor: impl PgAcquire<'_> + 'async_trait,
    ) -> Result<Option<ProjectRow>> {
        let mut conn = executor
            .acquire()
            .await
            .context("failed to acquire postgres connection")?;
        let row: Option<ProjectRow> =
        sqlx::query_as::<_, ProjectRow>(
            "SELECT id, name, description, COALESCE(config, '{}'::jsonb) AS config, created_at, updated_at
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use anyhow::Result;
    use sqlx::PgConnection;
    use sqlx::PgPool;
    use std::cmp::min;

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

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_create_and_find_by_id(pool: PgPool) -> Result<()> {
        let repo = PgProjectRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let project = create_project(&mut tx)
            .await
            .expect("new project should be created");
        let fetched = repo
            .find_by_id(&project.id(), &mut tx)
            .await
            .expect("inserted project should be found");
        if let Some(fetched) = fetched {
            assert_eq!(fetched.id, project.id().to_uuid());
            assert_eq!(fetched.name, project.name().as_str());
            assert_eq!(fetched.description, project.description().as_str());
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
    async fn test_create_and_find_by_name(pool: PgPool) -> Result<()> {
        let repo = PgProjectRepository;
        let mut tx = pool
            .begin()
            .await
            .expect("transaction should be started properly");
        let project = create_project(&mut tx)
            .await
            .expect("new project should be created");
        let fetched = repo
            .find_by_name(&project.name(), &mut tx)
            .await
            .expect("inserted project should be found");
        if let Some(fetched) = fetched {
            assert_eq!(fetched.id, project.id().to_uuid());
            assert_eq!(fetched.name, project.name().as_str());
            assert_eq!(fetched.description, project.description().as_str());
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
        let records = testutils::rand::i32(0, 200);
        for _ in 0..records {
            create_project(&mut tx)
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
        let records = testutils::rand::i32(0, 200);
        for _ in 0..records {
            create_project(&mut tx)
                .await
                .expect("new project should be created");
        }
        let limit = testutils::rand::i32(0, 200);
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
}

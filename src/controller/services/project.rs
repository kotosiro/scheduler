use crate::controller::domain::dtos::project::Project as ProjectRow;
use crate::controller::domain::dtos::project::ProjectConfig as ProjectConfigRow;
use crate::controller::domain::dtos::project::ProjectSummary as ProjectSummaryRow;
use crate::controller::domain::dtos::workflow::WorkflowSummary as WorkflowSummaryRow;
use crate::controller::domain::entities::project::Project;
use crate::controller::domain::entities::project::ProjectId;
use crate::controller::domain::entities::project::ProjectName;
use crate::controller::domain::entities::workflow::WorkflowName;
use crate::controller::domain::repositories::project::PgProjectRepository;
use crate::controller::domain::repositories::project::ProjectRepository;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;

#[async_trait]
pub trait ProjectService {
    async fn create(&self, project: &Project) -> Result<PgQueryResult>;

    async fn delete(&self, id: &ProjectId) -> Result<PgQueryResult>;

    async fn list(&self, limit: Option<i64>) -> Result<Vec<ProjectRow>>;

    async fn get_by_id(&self, id: &ProjectId) -> Result<Option<ProjectRow>>;

    async fn get_by_name(&self, name: &ProjectName) -> Result<Option<ProjectRow>>;

    async fn get_summary_by_id(&self, id: &ProjectId) -> Result<Option<ProjectSummaryRow>>;

    async fn get_config_by_id(&self, id: &ProjectId) -> Result<Option<ProjectConfigRow>>;

    async fn list_workflows_by_id(
        &self,
        id: &ProjectId,
        name: Option<&WorkflowName>,
        after: Option<&WorkflowName>,
        limit: Option<i64>,
    ) -> Result<Vec<WorkflowSummaryRow>>;
}

#[async_trait]
impl ProjectService for PgPool {
    async fn create(&self, project: &Project) -> Result<PgQueryResult> {
        let repo = PgProjectRepository;
        repo.create(project, self).await
    }

    async fn delete(&self, id: &ProjectId) -> Result<PgQueryResult> {
        let repo = PgProjectRepository;
        repo.delete(id, self).await
    }

    async fn list(&self, limit: Option<i64>) -> Result<Vec<ProjectRow>> {
        let repo = PgProjectRepository;
        repo.list(limit, self).await
    }

    async fn get_by_id(&self, id: &ProjectId) -> Result<Option<ProjectRow>> {
        let repo = PgProjectRepository;
        repo.get_by_id(id, self).await
    }

    async fn get_by_name(&self, name: &ProjectName) -> Result<Option<ProjectRow>> {
        let repo = PgProjectRepository;
        repo.get_by_name(name, self).await
    }

    async fn get_summary_by_id(&self, id: &ProjectId) -> Result<Option<ProjectSummaryRow>> {
        let repo = PgProjectRepository;
        repo.get_summary_by_id(id, self).await
    }

    async fn get_config_by_id(&self, id: &ProjectId) -> Result<Option<ProjectConfigRow>> {
        let repo = PgProjectRepository;
        repo.get_config_by_id(id, self).await
    }

    async fn list_workflows_by_id(
        &self,
        id: &ProjectId,
        name: Option<&WorkflowName>,
        after: Option<&WorkflowName>,
        limit: Option<i64>,
    ) -> Result<Vec<WorkflowSummaryRow>> {
        let repo = PgProjectRepository;
        repo.list_workflows_by_id(id, name, after, limit, self)
            .await
    }
}
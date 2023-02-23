use crate::controller::domain::entities::project::Project;
use crate::controller::domain::entities::project::ProjectId;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait ProjectRepository: Send + Sync + 'static {
    async fn create(&self, project: &Project) -> Result<()>;

    async fn update(&self, project: &Project) -> Result<()>;

    async fn find_by_id(&self, id: &ProjectId) -> Result<Option<Project>>;
}

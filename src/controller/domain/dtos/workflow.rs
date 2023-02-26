use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub project_id: Uuid,
    pub description: String,
    pub paused: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

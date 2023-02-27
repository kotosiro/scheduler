use chrono::DateTime;
use chrono::Utc;
use serde_json::Value as Json;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub config: Option<Json>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
#[serde(transparent)]
pub struct ProjectConfig(pub Json);

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct ProjectSummary {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub workflows: i64,
    pub running_jobs: i64,
    pub waiting_jobs: i64,
    pub fails_last_hour: i64,
    pub successes_last_hour: i64,
    pub errors_last_hour: i64,
}

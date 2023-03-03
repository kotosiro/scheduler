use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub name: String,
    pub workflow_id: Uuid,
    pub threshold: i32,
    pub image: String,
    pub args: Vec<String>,
    pub envs: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

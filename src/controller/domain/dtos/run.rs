use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct Run {
    pub id: Uuid,
    pub state: String,
    pub priority: String,
    pub job_id: Uuid,
    pub triggered_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

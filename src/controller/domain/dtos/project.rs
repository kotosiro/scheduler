use chrono::DateTime;
use chrono::Utc;
use serde_json::Value as Json;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub config: Option<Json>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

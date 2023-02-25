use chrono::NaiveDateTime;
use serde_json::Value as Json;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub config: Option<Json>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

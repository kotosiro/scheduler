use super::job::JobId;
use crate::impl_uuid_property;
use crate::messages::run::RunPriority;
use crate::messages::token::TokenState;
use anyhow::anyhow;
use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use getset::Getters;
use getset::Setters;
use serde_json::json;
use serde_json::Value as Json;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunId {
    value: Uuid,
}

impl_uuid_property!(RunId);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct Run {
    #[getset(get = "pub")]
    id: RunId,
    #[getset(get = "pub", set = "pub")]
    state: TokenState,
    #[getset(get = "pub", set = "pub")]
    priority: RunPriority,
    #[getset(get = "pub", set = "pub")]
    job_id: JobId,
    #[getset(get = "pub", set = "pub")]
    triggered_at: DateTime<Utc>,
}

impl Run {
    pub fn new(
        id: String,
        state: TokenState,
        priority: RunPriority,
        job_id: String,
        triggered_at: DateTime<Utc>,
    ) -> Result<Self> {
        Ok(Self {
            id: RunId::try_from(id)?,
            state: state,
            priority: priority,
            job_id: JobId::try_from(job_id)?,
            triggered_at: triggered_at,
        })
    }
}

impl TryFrom<Json> for Run {
    type Error = anyhow::Error;

    fn try_from(json: Json) -> std::result::Result<Self, Self::Error> {
        let id = json!(Uuid::new_v4().to_string());
        let id = match &json["id"] {
            Json::Null => &id,
            value => value,
        };
        let triggered_at = match &json["id"] {
            Json::Null => return Err(anyhow!("invalid json value")),
            value => {
                let value = value.as_str().ok_or(anyhow!("invalid json value"))?;
                DateTime::from_str(value)?
            }
        };
        Ok(Self {
            id: RunId::try_from(id)?,
            state: TokenState::try_from(&json["state"])?,
            priority: RunPriority::try_from(&json["priority"])?,
            job_id: JobId::try_from(&json["job_id"])?,
            triggered_at: triggered_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_run_id() {
        assert!(matches!(RunId::try_from(testutils::rand::uuid()), Ok(_)));
    }

    #[test]
    fn test_invalid_run_id() {
        assert!(matches!(
            RunId::try_from(testutils::rand::string(255)),
            Err(_)
        ));
    }
}

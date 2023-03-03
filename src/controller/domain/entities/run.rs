use super::job::JobId;
use crate::impl_uuid_property;
use crate::messages::run::RunPriority;
use crate::messages::token::TokenState;
use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use getset::Getters;
use getset::Setters;
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

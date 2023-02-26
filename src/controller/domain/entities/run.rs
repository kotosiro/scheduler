use super::job::JobId;
use super::token::TokenState;
use crate::impl_uuid_property;
use anyhow::Result;
use getset::Getters;
use getset::Setters;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunId {
    value: Uuid,
}

impl_uuid_property!(RunId);

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
    strum_macros::EnumString,
)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "VARCHAR")]
pub enum RunPriority {
    #[strum(ascii_case_insensitive)]
    BackFill = 0,
    #[strum(ascii_case_insensitive)]
    Low = 1,
    #[strum(ascii_case_insensitive)]
    Normal = 2,
    #[strum(ascii_case_insensitive)]
    High = 3,
}

impl RunPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            RunPriority::BackFill => "backfill",
            RunPriority::Low => "low",
            RunPriority::Normal => "normal",
            RunPriority::High => "high",
        }
    }
}

impl Default for RunPriority {
    fn default() -> Self {
        Self::Normal
    }
}

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
}

impl Run {
    pub fn new(
        id: String,
        state: TokenState,
        priority: RunPriority,
        job_id: String,
    ) -> Result<Self> {
        Ok(Self {
            id: RunId::try_from(id)?,
            state: state,
            priority: priority,
            job_id: JobId::try_from(job_id)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

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

    #[test]
    fn test_valid_run_priority() {
        let candidates = vec!["BackFill", "Low", "Normal", "High"];
        let priority = testutils::rand::choice(&candidates);
        assert!(matches!(RunPriority::from_str(priority), Ok(_)));
    }

    #[test]
    fn test_invalid_run_priority() {
        let candidates = vec!["Apple", "Orange", "Strawberry", "Grape"];
        let priority = testutils::rand::choice(&candidates);
        assert!(matches!(RunPriority::from_str(priority), Err(_)));
    }
}

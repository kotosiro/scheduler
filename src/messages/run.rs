use anyhow::anyhow;
use anyhow::Context;
use serde_json::Value as Json;

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

impl AsRef<str> for RunPriority {
    fn as_ref(&self) -> &str {
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

impl TryFrom<&Json> for RunPriority {
    type Error = anyhow::Error;

    fn try_from(value: &Json) -> std::result::Result<Self, <RunPriority as TryFrom<&Json>>::Error> {
        let value = value.as_str().ok_or(anyhow!("invalid json value"))?;
        Ok(<RunPriority as std::str::FromStr>::from_str(value)
            .context("failed to parse run priority")?)
    }
}

impl TryFrom<Json> for RunPriority {
    type Error = anyhow::Error;

    fn try_from(value: Json) -> std::result::Result<Self, <RunPriority as TryFrom<Json>>::Error> {
        let value = value.as_str().ok_or(anyhow!("invalid json value"))?;
        Ok(<RunPriority as std::str::FromStr>::from_str(value)
            .context("failed to parse run priority")?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

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

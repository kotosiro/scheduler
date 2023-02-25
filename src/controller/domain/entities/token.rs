use super::job::JobId;
use crate::impl_i32_property;
use chrono::NaiveDateTime;
use getset::Getters;
use getset::Setters;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct TokenCount {
    #[validate(range(min = 0))]
    value: i32,
}

impl_i32_property!(TokenCount);

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
pub enum TokenState {
    #[strum(ascii_case_insensitive)]
    Waiting,
    #[strum(ascii_case_insensitive)]
    Active,
    #[strum(ascii_case_insensitive)]
    Running,
    #[strum(ascii_case_insensitive)]
    Success,
    #[strum(ascii_case_insensitive)]
    Failure,
    #[strum(ascii_case_insensitive)]
    Error,
}

impl TokenState {
    pub fn is_final(&self) -> bool {
        matches!(
            self,
            TokenState::Success | TokenState::Failure | TokenState::Error
        )
    }
}

impl AsRef<str> for TokenState {
    fn as_ref(&self) -> &str {
        match self {
            TokenState::Waiting => "waiting",
            TokenState::Active => "active",
            TokenState::Running => "running",
            TokenState::Success => "success",
            TokenState::Failure => "failure",
            TokenState::Error => "error",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct Token {
    #[getset(get = "pub", set = "pub")]
    job_id: JobId,
    #[getset(get = "pub", set = "pub")]
    count: TokenCount,
    #[getset(get = "pub", set = "pub")]
    state: TokenState,
    #[getset(get = "pub", set = "pub")]
    created_at: Option<NaiveDateTime>,
    #[getset(get = "pub", set = "pub")]
    updated_at: Option<NaiveDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_valid_token_count() {
        assert!(matches!(
            TokenCount::new(testutils::rand::i32(0, 100)),
            Ok(_)
        ));
    }

    #[test]
    fn test_invalid_token_count() {
        assert!(matches!(
            TokenCount::new(testutils::rand::i32(-1000, -1)),
            Err(_)
        ));
    }

    #[test]
    fn test_valid_token_state() {
        let candidates = vec![
            "Waiting", "Active", "Running", "Success", "Failure", "Error",
        ];
        let state = testutils::rand::choice(&candidates);
        assert!(matches!(TokenState::from_str(state), Ok(_)));
    }

    #[test]
    fn test_invalid_token_state() {
        let candidates = vec!["Apple", "Orange", "Strawberry", "Grape"];
        let state = testutils::rand::choice(&candidates);
        assert!(matches!(TokenState::from_str(state), Err(_)));
    }
}

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
    pub fn is_done(&self) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

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

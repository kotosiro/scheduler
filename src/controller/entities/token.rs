use super::job::JobId;
use crate::impl_i32_property;
use crate::messages::token::TokenState;
use anyhow::Result;
use getset::Getters;
use getset::Setters;
use serde_json::Value as Json;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct TokenCount {
    #[validate(range(min = 0))]
    value: i32,
}

impl_i32_property!(TokenCount);

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters)]
pub struct Token {
    #[getset(get = "pub", set = "pub")]
    job_id: JobId,
    #[getset(get = "pub", set = "pub")]
    count: TokenCount,
    #[getset(get = "pub", set = "pub")]
    state: TokenState,
}

impl Token {
    pub fn new(job_id: String, count: i32, state: TokenState) -> Result<Self> {
        Ok(Self {
            job_id: JobId::try_from(job_id)?,
            count: TokenCount::new(count)?,
            state: state,
        })
    }
}

impl TryFrom<Json> for Token {
    type Error = anyhow::Error;

    fn try_from(json: Json) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            job_id: JobId::try_from(&json["job_id"])?,
            count: TokenCount::try_from(&json["count"])?,
            state: TokenState::try_from(&json["state"])?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

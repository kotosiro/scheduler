use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Validation(String),

    #[error(r#"{entity_type} was not found for id "{id}"."#)]
    NotFound {
        entity_type: &'static str,
        id: String,
    },

    #[error(transparent)]
    Infrastructure(anyhow::Error),

    #[error("{0}")]
    Unexpected(String),
}

impl From<ValidationErrors> for Error {
    fn from(error: ValidationErrors) -> Self {
        Error::Validation(error.to_string())
    }
}

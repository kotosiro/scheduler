use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
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

impl From<ValidationErrors> for DomainError {
    fn from(error: ValidationErrors) -> Self {
        DomainError::Validation(error.to_string())
    }
}

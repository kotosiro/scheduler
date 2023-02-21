#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error(transparent)]
    Build(#[from] config::ConfigError),
    #[error(transparent)]
    Serialization(#[from] anyhow::Error),
}

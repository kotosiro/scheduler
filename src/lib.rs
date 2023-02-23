pub mod config;
pub mod controller;
mod infra;
pub mod tracing;

pub const VERSION: &str = git_version::git_version!();

//pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("migrations");

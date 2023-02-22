pub mod config;
pub mod controller;
mod domain;
mod infra;
pub mod tracing;

pub const VERSION: &str = git_version::git_version!();

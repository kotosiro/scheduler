pub mod config;
pub mod controller;
pub mod domain;
mod infrastructure;
pub mod logging;

pub const VERSION: &str = git_version::git_version!();

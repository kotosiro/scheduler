pub mod config;
pub mod controller;
mod infrastructure;
pub mod logging;

pub const VERSION: &str = git_version::git_version!();

pub mod config;
pub mod controller;
pub mod logging;
pub mod mq;

pub const VERSION: &str = git_version::git_version!();

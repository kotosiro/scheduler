#![allow(dead_code)]
pub mod config;
pub mod controller;
pub mod logging;
pub mod messages;
mod middlewares;

pub const VERSION: &str = git_version::git_version!();

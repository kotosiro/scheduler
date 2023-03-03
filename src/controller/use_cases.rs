pub mod api;
pub mod internal;
use crate::controller::Controller;
use anyhow::Context;
use anyhow::Result;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tracing::debug;

async fn root() -> &'static str {
    "Hello, World!"
}

async fn route(controller: Arc<Controller>) -> Result<Router> {
    let app = Router::new().route("/", get(root));
    Ok(app)
}

pub async fn bind(controller: Arc<Controller>) -> Result<()> {
    let app = route(controller.clone())
        .await
        .context("failed to create axum router")?;
    let addr = controller
        .config
        .controller_bind
        .as_str()
        .parse()
        .context(format!(
            r#"failed to parse "{}" to SocketAddr"#,
            controller.config.controller_bind
        ))?;
    debug!("kotosiro controller listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context(format!(
            r#"failed to bind "{}" to hyper::Server"#,
            controller.config.controller_bind,
        ))?;
    Ok(())
}

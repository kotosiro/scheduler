pub mod api;
pub mod internal;
use crate::controller::services::config;
use crate::controller::Controller;
use anyhow::Context;
use anyhow::Result;
use axum::routing::get;
use axum::Router;
use lapin::Channel;
use std::sync::Arc;
use tracing::debug;

pub struct State {
    mq_chan: Channel,
    controller: Arc<Controller>,
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn route(controller: Arc<Controller>) -> Result<Router> {
    let mq_chan = controller
        .mq_conn
        .create_channel()
        .await
        .context("failed to create rabbitmq channel")?;
    let state = Arc::new(State {
        mq_chan,
        controller,
    });
    config::setup(&state.mq_chan)
        .await
        .context("failed to setup config cache")?;
    let app = Router::new().route("/", get(root)).with_state(state);
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

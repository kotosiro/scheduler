pub mod api;
pub mod internal;
use crate::controller::services::config::ConfigService;
use crate::controller::Controller;
use crate::messages::opa::Token;
use anyhow::Context;
use anyhow::Result;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::middleware::from_extractor;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::routing::post;
use axum::routing::put;
use axum::Json;
use axum::Router;
use lapin::Channel;
use serde_json::json;
use std::sync::Arc;
use tracing::debug;

pub struct State {
    mq_chan: Channel,
    controller: Arc<Controller>,
}

type SharedState = Arc<State>;

pub enum UseCaseError {
    InternalServerProblem(anyhow::Error),
    BadRequest,
    Unauthorized,
    ValidationFailed,
    Conflict,
}

impl From<anyhow::Error> for UseCaseError {
    fn from(e: anyhow::Error) -> Self {
        UseCaseError::InternalServerProblem(e)
    }
}

impl IntoResponse for UseCaseError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            UseCaseError::InternalServerProblem(e) => {
                debug!("stacktrace: {}", e.backtrace());
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
            }
            UseCaseError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            UseCaseError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            UseCaseError::ValidationFailed => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Validation errors")
            }
            UseCaseError::Conflict => (StatusCode::CONFLICT, "Confliction occured"),
        };
        let body = Json(json!({
            "error": message,
        }));
        (status, body).into_response()
    }
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
    ConfigService::setup(&state.mq_chan)
        .await
        .context("failed to setup config service")?;
    let app = Router::new()
        .route(
            "/api/project",
            get(self::api::project::get_by_name)
                .post(self::api::project::create)
                .put(self::api::project::create),
        )
        .layer(Extension(state))
        .layer(from_extractor::<Token>());
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

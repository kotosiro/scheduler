use crate::controller::domain::entities::project::Project;
use crate::controller::services::config::ConfigService;
use crate::controller::services::opa::Event;
use crate::controller::services::opa::OPAService;
use crate::controller::services::project::ProjectService;
use crate::controller::use_cases::SharedState;
use crate::controller::use_cases::UseCaseError;
use crate::messages::config::ConfigUpdate;
use crate::messages::opa::Token;
use crate::middlewares::postgres::has_conflict;
use crate::middlewares::postgres::maybe_conflict;
use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use serde_json::Value;
use tracing::error;
use tracing::info;
use tracing::warn;
use uuid::Uuid;

pub async fn create(
    token: Token,
    Extension(state): Extension<SharedState>,
    Json(payload): Json<Value>,
) -> Result<impl IntoResponse, UseCaseError> {
    let project = if let Ok(project) = Project::new(
        payload["id"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or(Uuid::new_v4().to_string()),
        payload["name"].as_str().unwrap_or("").to_string(),
        payload["description"].as_str().unwrap_or("").to_string(),
        payload.get("config").cloned(),
    ) {
        project
    } else {
        error!("invalid project specification found");
        return Err(UseCaseError::ValidationFailed);
    };
    if let Err(_) = OPAService::authorize(
        &state.controller.db_pool,
        state.controller.config.no_auth,
        &state.controller.config.opa_addr,
        Event::update()
            .on_project(Some(project.id().to_uuid()))
            .with_token(token),
    )
    .await
    {
        warn!("failed to update project");
        return Err(UseCaseError::Unauthorized);
    }
    let res = ProjectService::create(&state.controller.db_pool, &project).await;
    match maybe_conflict(res)? {
        Ok(_) => {
            info!(
                r#"updated project id: "{}" name: "{}""#,
                project.id().as_uuid(),
                project.name().as_str()
            );
            if let Err(_) = ConfigService::publish(
                &state.mq_chan,
                ConfigUpdate::Project(project.id().to_uuid()),
            )
            .await
            {
                warn!("failed to publish project config update");
            }
            let body = Json(json!({
                "id": project.id().as_uuid(),
                "name": project.name().as_str(),
                "description": project.description().as_str(),
            }));
            Ok((StatusCode::CREATED, body).into_response())
        }
        Err(e) if has_conflict(&e) => {
            warn!("failed to update project: {}", e);
            Err(UseCaseError::Conflict)
        }
        _ => Err(UseCaseError::InternalServerProblem(anyhow!(
            "Internal server error"
        ))),
    }
}

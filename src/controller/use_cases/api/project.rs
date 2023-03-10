use crate::controller::domain::entities::project::Project;
use crate::controller::domain::entities::project::ProjectName;
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
use axum::extract::Query;
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
    match maybe_conflict(ProjectService::create(&state.controller.db_pool, &project).await)? {
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

#[derive(serde::Deserialize)]
pub struct GetByNameQuery {
    name: Option<String>,
}

pub async fn get_by_name(
    token: Token,
    Extension(state): Extension<SharedState>,
    query: Query<GetByNameQuery>,
) -> Result<impl IntoResponse, UseCaseError> {
    if let Some(name) = &query.name {
        let name = if let Ok(name) = ProjectName::new(name) {
            name
        } else {
            error!("invalid project name found");
            return Err(UseCaseError::ValidationFailed);
        };
        match ProjectService::get_by_name(&state.controller.db_pool, &name).await? {
            None => Ok(StatusCode::NOT_FOUND.into_response()),
            Some(project) => {
                if let Err(_) = OPAService::authorize(
                    &state.controller.db_pool,
                    state.controller.config.no_auth,
                    &state.controller.config.opa_addr,
                    Event::get().on_project(Some(project.id)).with_token(token),
                )
                .await
                {
                    warn!("failed to get project");
                    return Err(UseCaseError::Unauthorized);
                }
                let body = Json(json!({
                    "id": project.id,
                    "name": project.name,
                    "description": project.description,
                    "config": project.config,
                    "created_at": project.created_at,
                    "updated_at": project.updated_at,
                }));
                Ok((StatusCode::OK, body).into_response())
            }
        }
    } else {
        if let Err(_) = OPAService::authorize(
            &state.controller.db_pool,
            state.controller.config.no_auth,
            &state.controller.config.opa_addr,
            Event::list().with_token(token),
        )
        .await
        {
            warn!("failed to list project");
            return Err(UseCaseError::Unauthorized);
        }
        let projects = ProjectService::list(&state.controller.db_pool, None).await?;
        let body: Json<Value> = Json(Value::Array(
            projects
                .into_iter()
                .map(|p| {
                    json!({
                        "id": p.id,
                        "name": p.name,
                        "description": p.description,
                        "config": p.config,
                        "created_at": p.created_at,
                        "updated_at": p.updated_at,
                    })
                })
                .collect(),
        ));
        Ok((StatusCode::OK, body).into_response())
    }
}

use crate::controller::entities::project::Project;
use crate::controller::entities::project::ProjectId;
use crate::controller::entities::project::ProjectName;
use crate::controller::entities::workflow::WorkflowName;
use crate::controller::interactors::InteractorError;
use crate::controller::interactors::SharedState;
use crate::controller::services::config::ConfigService;
use crate::controller::services::opa::Event;
use crate::controller::services::opa::OPAService;
use crate::controller::services::project::ProjectService;
use crate::infra::opa::Token;
use crate::infra::postgres::has_conflict;
use crate::infra::postgres::pg_error;
use crate::messages::config::ConfigUpdate;
use anyhow::anyhow;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde_json::json;
use serde_json::Value;
use tracing::error;
use tracing::info;
use tracing::warn;

#[derive(serde::Deserialize)]
pub struct GetByNameQuery {
    name: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ListWorkflowsByIdQuery {
    name: Option<String>,
    after: Option<String>,
    limit: Option<i64>,
}

#[derive(serde::Deserialize)]
pub struct CreateJson {
    id: Option<String>,
    name: String,
    description: String,
    config: Option<Value>,
}

pub async fn create(
    token: Token,
    Extension(state): Extension<SharedState>,
    Json(payload): Json<CreateJson>,
) -> Result<Response, InteractorError> {
    let id = payload.id.unwrap_or(uuid::Uuid::new_v4().to_string());
    let project =
        if let Ok(project) = Project::new(id, payload.name, payload.description, payload.config) {
            project
        } else {
            error!("invalid project specification found");
            return Err(InteractorError::ValidationFailed);
        };
    if let Err(_) = OPAService::authorize(
        &state.controller.db_pool,
        &state.controller.config.no_auth,
        state.controller.config.opa_addr.as_ref(),
        Event::update()
            .on_project(project.id().to_uuid())
            .with_token(token),
    )
    .await
    {
        warn!("failed to update project");
        return Err(InteractorError::Unauthorized);
    }
    match pg_error(ProjectService::create(&state.controller.db_pool, &project).await)? {
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
            Err(InteractorError::Conflict)
        }
        _ => Err(InteractorError::InternalServerProblem(anyhow!(
            "Internal server error"
        ))),
    }
}

pub async fn get_by_name(
    token: Token,
    Extension(state): Extension<SharedState>,
    query: Query<GetByNameQuery>,
) -> Result<Response, InteractorError> {
    if let Some(name) = &query.name {
        let name = if let Ok(name) = ProjectName::new(name) {
            name
        } else {
            error!("invalid project name found");
            return Err(InteractorError::ValidationFailed);
        };
        match ProjectService::get_by_name(&state.controller.db_pool, &name).await? {
            None => Ok(StatusCode::NOT_FOUND.into_response()),
            Some(row) => {
                if let Err(_) = OPAService::authorize(
                    &state.controller.db_pool,
                    &state.controller.config.no_auth,
                    state.controller.config.opa_addr.as_ref(),
                    Event::get().on_project(row.id).with_token(token),
                )
                .await
                {
                    warn!("failed to get project");
                    return Err(InteractorError::Unauthorized);
                }
                let body = Json(json!({
                    "id": row.id,
                    "name": row.name,
                    "description": row.description,
                    "config": row.config,
                    "created_at": row.created_at,
                    "updated_at": row.updated_at,
                }));
                Ok((StatusCode::OK, body).into_response())
            }
        }
    } else {
        if let Err(_) = OPAService::authorize(
            &state.controller.db_pool,
            &state.controller.config.no_auth,
            state.controller.config.opa_addr.as_ref(),
            Event::list().with_token(token),
        )
        .await
        {
            warn!("failed to list project");
            return Err(InteractorError::Unauthorized);
        }
        let rows = ProjectService::list(&state.controller.db_pool, None).await?;
        let body: Json<Value> = Json(Value::Array(
            rows.into_iter()
                .map(|row| {
                    json!({
                        "id": row.id,
                        "name": row.name,
                        "description": row.description,
                        "config": row.config,
                        "created_at": row.created_at,
                        "updated_at": row.updated_at,
                    })
                })
                .collect(),
        ));
        Ok((StatusCode::OK, body).into_response())
    }
}

pub async fn get_summary_by_id(
    token: Token,
    Extension(state): Extension<SharedState>,
    Path(id): Path<String>,
) -> Result<Response, InteractorError> {
    let id = if let Ok(id) = ProjectId::try_from(id) {
        id
    } else {
        error!("project id must be uuid v4");
        return Err(InteractorError::BadRequest);
    };
    match ProjectService::get_summary_by_id(&state.controller.db_pool, &id).await? {
        None => Ok(StatusCode::NOT_FOUND.into_response()),
        Some(row) => {
            if let Err(_) = OPAService::authorize(
                &state.controller.db_pool,
                &state.controller.config.no_auth,
                state.controller.config.opa_addr.as_ref(),
                Event::get().on_project(row.id).with_token(token),
            )
            .await
            {
                warn!("failed to get project");
                return Err(InteractorError::Unauthorized);
            }
            let body = Json(json!({
                "id": row.id,
                "name": row.name,
                "description": row.description,
                "workflows": row.workflows,
                "running_jobs": row.running_jobs,
                "waiting_jobs": row.waiting_jobs,
                "fails_last_hour": row.fails_last_hour,
                "successes_last_hour": row.successes_last_hour,
                "errors_last_hour": row.errors_last_hour,
            }));
            Ok((StatusCode::OK, body).into_response())
        }
    }
}

pub async fn delete(
    token: Token,
    Extension(state): Extension<SharedState>,
    Path(id): Path<String>,
) -> Result<Response, InteractorError> {
    let id = if let Ok(id) = ProjectId::try_from(id) {
        id
    } else {
        error!("project id must be uuid v4");
        return Err(InteractorError::BadRequest);
    };
    if let Err(_) = OPAService::authorize(
        &state.controller.db_pool,
        &state.controller.config.no_auth,
        state.controller.config.opa_addr.as_ref(),
        Event::delete().on_project(id.to_uuid()).with_token(token),
    )
    .await
    {
        warn!("failed to delete project");
        return Err(InteractorError::Unauthorized);
    }
    match pg_error(ProjectService::delete(&state.controller.db_pool, &id).await)? {
        Ok(done) => {
            if done.rows_affected() == 1 {
                info!(r#"deleted project id: "{}""#, id.as_uuid());
                Ok(StatusCode::NO_CONTENT.into_response())
            } else {
                info!(r#"no project was found with id: "{}""#, id.as_uuid());
                Ok(StatusCode::NOT_FOUND.into_response())
            }
        }
        Err(e) => {
            warn!("failed to delete project: {}", e);
            Err(InteractorError::InternalServerProblem(anyhow!(
                "Internal server error"
            )))
        }
    }
}

pub async fn list_workflows_by_id(
    token: Token,
    Extension(state): Extension<SharedState>,
    Path(id): Path<String>,
    query: Query<ListWorkflowsByIdQuery>,
) -> Result<Response, InteractorError> {
    let id = if let Ok(id) = ProjectId::try_from(id) {
        id
    } else {
        error!("project id must be uuid v4");
        return Err(InteractorError::BadRequest);
    };
    let name = query
        .name
        .as_ref()
        .map(WorkflowName::new)
        .transpose()
        .unwrap_or(None);
    let after = query
        .after
        .as_ref()
        .map(WorkflowName::new)
        .transpose()
        .unwrap_or(None);
    let limit = query.limit;
    if let Err(_) = OPAService::authorize(
        &state.controller.db_pool,
        &state.controller.config.no_auth,
        state.controller.config.opa_addr.as_ref(),
        Event::list().on_project(id.to_uuid()).with_token(token),
    )
    .await
    {
        warn!("failed to list project workflows");
        return Err(InteractorError::Unauthorized);
    }
    let rows = ProjectService::list_workflows_by_id(
        &state.controller.db_pool,
        &id,
        name.as_ref(),
        after.as_ref(),
        limit.as_ref(),
    )
    .await?;
    let body: Json<Value> = Json(Value::Array(
        rows.into_iter()
            .map(|row| {
                json!({
                    "id": row.id,
                    "name": row.name,
                    "description": row.description,
                    "paused": row.paused,
                    "success": row.success,
                    "running": row.running,
                    "failure": row.failure,
                    "waiting": row.waiting,
                    "error": row.error,
                })
            })
            .collect(),
    ));
    Ok((StatusCode::OK, body).into_response())
}

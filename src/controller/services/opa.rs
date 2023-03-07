use crate::controller::domain::entities::workflow::WorkflowId;
use crate::controller::domain::repositories::workflow::PgWorkflowRepository;
use crate::controller::domain::repositories::workflow::WorkflowRepository;
use crate::messages::opa::Action;
use crate::messages::opa::Decision;
use crate::messages::opa::Input;
use crate::messages::opa::Query;
use crate::messages::opa::Resource;
use crate::messages::opa::Token;
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::extract::TypedHeader;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::RequestPartsExt;
use reqwest::Url;
use serde_json::json;
use sqlx::PgPool;
use tracing::debug;
use tracing::error;
use tracing::warn;
use uuid::Uuid;

#[derive(Debug)]
pub enum TokenError {
    NeverThrown,
}

#[async_trait]
impl<B> FromRequestParts<B> for Token
where
    B: Send + Sync,
{
    type Rejection = TokenError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &B,
    ) -> std::result::Result<Self, Self::Rejection> {
        let maybe = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .ok();
        match maybe {
            Some(TypedHeader(Authorization(bearer))) => {
                Ok(Token::Bearer(bearer.token().to_owned()))
            }
            _ => Ok(Token::None),
        }
    }
}

impl IntoResponse for TokenError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            TokenError::NeverThrown => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug)]
pub struct Event {
    token: Token,
    action: Action,
    resource: Resource,
}

impl Event {
    pub fn get() -> Self {
        Self {
            token: Token::None,
            action: Action::Get,
            resource: Default::default(),
        }
    }

    pub fn list() -> Self {
        Self {
            token: Token::None,
            action: Action::List,
            resource: Default::default(),
        }
    }

    pub fn update() -> Self {
        Self {
            token: Token::None,
            action: Action::Update,
            resource: Default::default(),
        }
    }

    pub fn delete() -> Self {
        Self {
            token: Token::None,
            action: Action::Delete,
            resource: Default::default(),
        }
    }

    pub fn with_token(mut self, token: impl Into<Token>) -> Self {
        self.token = token.into();
        self
    }

    pub fn on_project(mut self, id: impl Into<Option<Uuid>>) -> Self {
        self.resource.project_id = id.into();
        self.resource.kind = "project".to_owned();
        self
    }

    pub fn on_workflow(
        mut self,
        id: impl Into<Option<Uuid>>,
        project_id: impl Into<Option<Uuid>>,
    ) -> Self {
        self.resource.workflow_id = id.into();
        self.resource.project_id = project_id.into();
        self.resource.kind = "workflow".to_owned();
        self
    }

    pub fn of_kind(mut self, kind: impl Into<String>) -> Self {
        self.resource.kind = kind.into();
        self
    }

    async fn is_authorized(&self, url: &Option<String>) -> Result<bool> {
        let opa = if let Some(opa) = url {
            opa
        } else {
            error!(
            "OPA sidecar address is unset (to disable auth you must set `KOTOSIRO_NO_AUTH=true`)"
        );
            return Ok(false);
        };
        let url = Url::parse(opa).context(format!(r#"failed to parse OPA url "{}""#, &opa))?;
        let url = url.join("/v1/data/kotosiro/authorize")?;
        let res = reqwest::Client::new()
            .post(url)
            .json(&Query {
                input: Input {
                    token: &self.token,
                    action: self.action,
                    resource: &self.resource,
                },
            })
            .send()
            .await
            .context(format!(r#"failed to query OPA request to "{}""#, &opa))?;
        let decision: Decision = res.json().await.context("failed to parse OPA response")?;
        if decision.result.unwrap_or(false) {
            debug!(?self.token, ?self.action, ?self.resource, "authorized");
        } else {
            warn!(?self.token, ?self.action, ?self.resource, "unauthorized");
        }
        Ok(decision.result.unwrap_or(false))
    }
}

#[async_trait]
pub trait OPAService {
    async fn authorize(&self, no_auth: bool, url: &Option<String>, mut event: Event) -> Result<()>;
}

#[async_trait]
impl OPAService for PgPool {
    async fn authorize(&self, no_auth: bool, url: &Option<String>, mut event: Event) -> Result<()> {
        if no_auth {
            return Ok(());
        }
        let repo = PgWorkflowRepository;
        if let Some(id) = event.resource.workflow_id {
            if event.resource.project_id.is_none() {
                let workflow_id = WorkflowId::new(id);
                let project_id = repo.get_project_id(&workflow_id, self).await?;
                event.resource.project_id = project_id;
            }
        }
        if event.is_authorized(url).await? {
            Ok(())
        } else {
            Err(anyhow!(r#"failed to authorize event "{:?}""#, event))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_authorized(pool: PgPool) {
        let no_auth: bool = testutils::rand::bool();
        let url: Option<String> = Some(String::from("http://127.0.0.1:8181"));
        OPAService::authorize(&pool, no_auth, &url, Event::get())
            .await
            .expect("GET access should be authorized");
        OPAService::authorize(&pool, no_auth, &url, Event::list())
            .await
            .expect("LIST access should be authorized");
    }

    #[sqlx::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_unauthorized(pool: PgPool) {
        let no_auth: bool = testutils::rand::bool();
        let url: Option<String> = Some(String::from("http://127.0.0.1:8181"));
        if no_auth {
            OPAService::authorize(&pool, no_auth, &url, Event::update())
                .await
                .expect("UPDATE access should be authorized");
            OPAService::authorize(&pool, no_auth, &url, Event::delete())
                .await
                .expect("DELETE access should be authorized");
        } else {
            let result = OPAService::authorize(&pool, no_auth, &url, Event::update()).await;
            assert!(matches!(result, Err(_)));
            let result = OPAService::authorize(&pool, no_auth, &url, Event::delete()).await;
            assert!(matches!(result, Err(_)));
        }
    }
}

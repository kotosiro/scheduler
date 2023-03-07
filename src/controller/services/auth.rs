use crate::config::Config;
use crate::messages::auth::Action;
use crate::messages::auth::Decision;
use crate::messages::auth::Input;
use crate::messages::auth::Query;
use crate::messages::auth::Resource;
use crate::messages::auth::Token;
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
use serde_json::json;
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

async fn query(config: &Config, token: Token, action: Action, resource: Resource) -> Result<bool> {
    let opa = if let Some(opa) = config.opa_addr.as_ref() {
        opa
    } else {
        error!(
            "OPA sidecar address is unset (to disable auth you must set `KOTOSIRO_NO_AUTH=true`)"
        );
        return Ok(false);
    };
    let url = opa.join("/v1/data/kotosiro/authorize")?;
    let res = reqwest::Client::new()
        .post(url)
        .json(&Query {
            input: Input {
                token: &token,
                action: action,
                resource: &resource,
            },
        })
        .send()
        .await
        .context(format!(r#"failed to query OPA request to "{}""#, &opa))?;
    let decision: Decision = res.json().await.context("failed to parse OPA response")?;
    if decision.result.unwrap_or(false) {
        debug!(?token, ?action, ?resource, "authorized");
    } else {
        warn!(?token, ?action, ?resource, "unauthorized");
    }
    Ok(decision.result.unwrap_or(false))
}

pub struct Checker {
    action: Action,
    resource: Resource,
}

impl Checker {
    pub fn project(mut self, id: impl Into<Option<Uuid>>) -> Self {
        self.resource.project_id = id.into();
        self.resource.kind = "project".to_owned();
        self
    }

    pub fn workflow(
        mut self,
        id: impl Into<Option<Uuid>>,
        project_id: impl Into<Option<Uuid>>,
    ) -> Self {
        self.resource.workflow_id = id.into();
        self.resource.project_id = project_id.into();
        self.resource.kind = "workflow".to_owned();
        self
    }

    pub fn kind(mut self, kind: impl Into<String>) -> Self {
        self.resource.kind = kind.into();
        self
    }

    pub async fn authorize(self, config: &Config, token: Token) -> Result<()> {
        if config.no_auth {
            return Ok(());
        }
        Ok(())
    }
}

pub fn get() -> Checker {
    Checker {
        action: Action::Get,
        resource: Default::default(),
    }
}

pub fn list() -> Checker {
    Checker {
        action: Action::List,
        resource: Default::default(),
    }
}

pub fn update() -> Checker {
    Checker {
        action: Action::Update,
        resource: Default::default(),
    }
}

pub fn delete() -> Checker {
    Checker {
        action: Action::Delete,
        resource: Default::default(),
    }
}

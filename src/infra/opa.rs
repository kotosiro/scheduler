use anyhow::Context;
use anyhow::Result;
use reqwest::Url;
use tracing::error;
use uuid::Uuid;

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::EnumString,
)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    #[strum(ascii_case_insensitive)]
    Get,
    #[strum(ascii_case_insensitive)]
    List,
    #[strum(ascii_case_insensitive)]
    Update,
    #[strum(ascii_case_insensitive)]
    Delete,
}

impl Action {
    pub fn is_read(&self) -> bool {
        matches!(self, Action::Get | Action::List)
    }
}

impl AsRef<str> for Action {
    fn as_ref(&self) -> &str {
        match self {
            Action::Get => "get",
            Action::List => "list",
            Action::Update => "update",
            Action::Delete => "delete",
        }
    }
}

#[derive(Debug, Default, serde::Serialize)]
pub struct Resource {
    pub project_id: Option<Uuid>,
    pub workflow_id: Option<Uuid>,
    pub kind: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum Token {
    Bearer(String),
    None,
}

#[derive(serde::Serialize)]
pub struct Input<'a> {
    pub action: Action,
    pub token: &'a Token,
    pub resource: &'a Resource,
}

#[derive(serde::Serialize)]
pub struct Query<'a> {
    pub input: Input<'a>,
}

#[derive(serde::Deserialize)]
pub struct Decision {
    pub result: Option<bool>,
}

pub async fn authorize<'a>(url: impl Into<Option<&String>>, query: &Query<'a>) -> Result<Decision> {
    let opa = if let Some(opa) = url.into() {
        opa
    } else {
        error!(
            "OPA sidecar address is unset (to disable auth you must set `KOTOSIRO_NO_AUTH=true`)"
        );
        return Ok(Decision {
            result: Some(false),
        });
    };
    let url = Url::parse(opa).context(format!(r#"failed to parse OPA url "{}""#, &opa))?;
    let url = url.join("/v1/data/kotosiro/authorize")?;
    let res = reqwest::Client::new()
        .post(url)
        .json(query)
        .send()
        .await
        .context(format!(r#"failed to query OPA request to "{}""#, &opa))?;
    let decision: Decision = res.json().await.context("failed to parse OPA response")?;
    Ok(decision)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_valid_action() {
        let candidates = vec!["Get", "List", "Update", "Delete"];
        let action = testutils::rand::choice(&candidates);
        assert!(matches!(Action::from_str(action), Ok(_)));
    }

    #[test]
    fn test_invalid_action() {
        let candidates = vec!["Apple", "Orange", "Strawberry", "Grape"];
        let action = testutils::rand::choice(&candidates);
        assert!(matches!(Action::from_str(action), Err(_)));
    }

    #[tokio::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_authorized() {
        let url: String = String::from("http://127.0.0.1:8181");
        let token: Token = Token::None;
        let action: Action = Action::Get;
        let resource: Resource = Default::default();
        let decision = authorize(
            &url,
            &Query {
                input: Input {
                    action: action,
                    token: &token,
                    resource: &resource,
                },
            },
        )
        .await
        .expect("decision should be returned");
        assert_eq!(decision.result.unwrap_or(false), true);
        let token: Token = Token::None;
        let action: Action = Action::List;
        let resource: Resource = Default::default();
        let decision = authorize(
            &url,
            &Query {
                input: Input {
                    action: action,
                    token: &token,
                    resource: &resource,
                },
            },
        )
        .await
        .expect("decision should be returned");
        assert_eq!(decision.result.unwrap_or(false), true);
    }

    #[tokio::test]
    #[ignore] // NOTE: Be sure '$ docker compose -f devops/local/docker-compose.yaml up' before running this test
    async fn test_unauthorized() {
        let url: String = String::from("http://127.0.0.1:8181");
        let token: Token = Token::None;
        let action: Action = Action::Update;
        let resource: Resource = Default::default();
        let decision = authorize(
            &url,
            &Query {
                input: Input {
                    action: action,
                    token: &token,
                    resource: &resource,
                },
            },
        )
        .await
        .expect("decision should be returned");
        assert_eq!(decision.result.unwrap_or(false), false);
        let url: String = String::from("http://127.0.0.1:8181");
        let token: Token = Token::None;
        let action: Action = Action::Delete;
        let resource: Resource = Default::default();
        let decision = authorize(
            &url,
            &Query {
                input: Input {
                    action: action,
                    token: &token,
                    resource: &resource,
                },
            },
        )
        .await
        .expect("decision should be returned");
        assert_eq!(decision.result.unwrap_or(false), false);
    }
}

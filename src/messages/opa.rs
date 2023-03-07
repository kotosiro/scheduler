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
    pub token: &'a Token,
    pub action: Action,
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
}

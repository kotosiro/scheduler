use uuid::Uuid;

pub const CONFIG_UPDATES_EXCHANGE: &str = "kotosiro.updates.config";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ConfigUpdate {
    Project(Uuid),
    Job(Uuid),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrap_project() {
        let uuid = Uuid::new_v4();
        let update = ConfigUpdate::Project(uuid.clone());
        if let ConfigUpdate::Project(unwraped) = update {
            assert_eq!(unwraped, uuid);
        }
    }

    #[test]
    fn test_unwrap_job() {
        let uuid = Uuid::new_v4();
        let update = ConfigUpdate::Job(uuid.clone());
        if let ConfigUpdate::Job(unwraped) = update {
            assert_eq!(unwraped, uuid);
        }
    }
}

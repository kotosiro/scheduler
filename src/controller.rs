use crate::config::Config;
//use anyhow::Result;
use lapin::Connection;
use sqlx::PgPool;
use uuid::Uuid;

pub struct Controller {
    pub id: Uuid,
    pub db_pool: PgPool,
    pub mq_conn: Connection,
    pub config: Config,
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_add() {
        assert_eq!(1 + 2, 3);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(1 + 2, 3);
    }
}

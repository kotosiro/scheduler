use anyhow::Result;
use sqlx::postgres::PgDatabaseError;
use sqlx::Executor;
use sqlx::PgPool;
use tracing::info;
use tracing::trace;

const INTEGRITY_ERROR: &str = "23";

pub async fn connect(db_url: &str) -> Result<PgPool> {
    info!("connecting to database");
    let pool = PgPool::connect(&db_url).await?;
    let mut conn = pool.acquire().await?;
    let done = conn.execute(include_str!("postgres/schema.sql")).await?;
    trace!("schema created: {} rows modified", done.rows_affected());
    info!("connected to database");
    Ok(pool)
}

pub fn error<T>(response: sqlx::Result<T>) -> Result<std::result::Result<T, Box<PgDatabaseError>>> {
    match response {
        Ok(v) => Ok(Ok(v)),
        Err(e) => match e {
            sqlx::Error::Database(e) => Ok(Err(e.downcast::<PgDatabaseError>())),
            e => Err(e.into()),
        },
    }
}

pub fn has_conflict(error: &PgDatabaseError) -> bool {
    &error.code()[..2] == INTEGRITY_ERROR
}

#[cfg(test)]
mod tests {
    use super::*;
    use testcontainers::clients;
    use testcontainers::images::postgres;
    use testutils;

    #[tokio::test]
    async fn test_connect() {
        let docker = clients::Cli::default();
        let node = docker.run(postgres::Postgres::default());
        let url = format!(
            "postgres://postgres:secret@127.0.0.1:{}",
            node.get_host_port_ipv4(5432)
        );

        println!("{}", &url);
        let pool = connect(&url)
            .await
            .expect("connection should be established");
        println!("{:?}", &pool);
    }
}

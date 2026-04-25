use snafu::ResultExt;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::infrastructure::error::PostgresMigrateSnafu;
use crate::infrastructure::error::{InfrastructureResult, PostgresSnafu};
use crate::infrastructure::internal::config::postgres_config::PostgresConfig;

#[derive(Debug, Clone)]
pub struct PostgresClient {
    pub connection: Pool<Postgres>,
}

impl PostgresClient {
    pub async fn new(config: &PostgresConfig) -> InfrastructureResult<Self> {
        let connection = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await
            .context(PostgresSnafu)?;
        sqlx::migrate!("./migrations")
            .run(&connection)
            .await
            .context(PostgresMigrateSnafu)?;
        Ok(PostgresClient { connection })
    }
}

use snafu::ResultExt;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::infrastructure::error::PostgresMigrateSnafu;
use crate::infrastructure::{
    config::postgres_config::PostgresConfig,
    error::{InfrastructureResult, PostgresSnafu},
};

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

use crate::infrastructure::config::surrealdb_config::SurrealDBConfig;
use anyhow::Context;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb_migrations::MigrationRunner;
use tracing::info;

#[derive(Debug, Clone)]
pub struct SurrealDBClient {
    pub client: Surreal<Client>,
}

impl SurrealDBClient {
    pub async fn new(config: &SurrealDBConfig) -> anyhow::Result<Self> {
        let client = Surreal::new::<Ws>(&config.address).await?;
        client
            .signin(Root {
                username: &config.root_name,
                password: &config.root_password,
            })
            .await
            .context("sign in to surrealdb as a root failed")?;
        client
            .use_ns(&config.namespace)
            .use_db(&config.database)
            .await
            .context("switch to namespace or database failed")?;
        MigrationRunner::new(&client)
            .up()
            .await
            .expect("Failed to apply migrations");
        info!("SurrealDB migrations applied successfully");
        Ok(SurrealDBClient { client })
    }
}

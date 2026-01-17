use crate::infrastructure::config::surreal::SurrealConfig;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

#[derive(Debug, Clone)]
pub struct SurrealClient {
    pub client: Surreal<Client>,
}

impl SurrealClient {
    pub async fn new(config: &SurrealConfig) -> Result<Self, surrealdb::Error> {
        let client = Surreal::new::<Ws>(&config.address).await?;
        client
            .signin(Root {
                username: &config.root_name,
                password: &config.root_password,
            })
            .await?;
        client
            .use_ns(&config.namespace)
            .use_db(&config.database)
            .await?;
        Ok(SurrealClient { client })
    }
}

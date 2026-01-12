use crate::infrastructure::config::surreal::SurrealConfig;
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;


#[derive(Debug, Clone)]
pub struct SurrealClient {
    pub client: Surreal<Client>
}

impl SurrealClient {
    pub async fn new(config: &SurrealConfig) -> Result<Self, surrealdb::Error> {
        let client = Surreal::new::<Http>(&config.address).await?;
        client.signin(
            Root {
                username: &config.root_name,
                password: &config.root_password
            }
        ).await?;
        client
            .use_ns(&config.namespace)
            .use_db(&config.database)
            .await?;
        Ok(SurrealClient {
            client
        })
    }
}

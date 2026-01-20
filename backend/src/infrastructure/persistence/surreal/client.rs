use crate::infrastructure::config::surreal::SurrealConfig;
use crate::infrastructure::persistence::surreal::auth_repository::SurrealAuthRepository;
use crate::infrastructure::persistence::surreal::health_repository::SurrealHealthRepository;
use crate::infrastructure::persistence::surreal::user_repository::SurrealUserRepository;
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
    pub fn health_repo() -> SurrealHealthRepository {
        SurrealHealthRepository::new()
    }
    pub fn user_repo(&self) -> SurrealUserRepository {
        SurrealUserRepository::new(self.clone())
    }
    pub fn auth_repo(&self) -> SurrealAuthRepository {
        SurrealAuthRepository::new(self.clone())
    }
}

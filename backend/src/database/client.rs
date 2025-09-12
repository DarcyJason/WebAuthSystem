use surrealdb::{Surreal, engine::remote::ws::Client};

#[derive(Debug, Clone)]
pub struct DBClient {
    pub surrealdb: Surreal<Client>,
}

impl DBClient {
    pub fn new(surrealdb: Surreal<Client>) -> Self {
        DBClient { surrealdb }
    }
}

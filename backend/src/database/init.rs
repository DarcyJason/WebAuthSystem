use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

use crate::{config::Config, errors::app_error::AppResult};

pub async fn init_surrealdb(config: Config) -> AppResult<Surreal<Client>> {
    let db = Surreal::new::<Ws>(config.surrealdb_server.surrealdb_host).await?;
    db.signin(Root {
        username: &config.surrealdb_server.surrealdb_root_name,
        password: &config.surrealdb_server.surrealdb_root_password,
    })
    .await?;
    db.use_ns(config.surrealdb_server.surrealdb_namespace)
        .use_db(config.surrealdb_server.surrealdb_database)
        .await?;
    Ok(db)
}

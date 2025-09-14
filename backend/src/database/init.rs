use std::time::Duration;

use ntex::time::interval;
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};
use tracing::{error, info};

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
    spawn_token_cleaner(db.clone());
    Ok(db)
}

fn spawn_token_cleaner(db: Surreal<Client>) {
    ntex::rt::spawn(async move {
        let ticker = interval(Duration::from_secs(3600));

        loop {
            ticker.tick().await;

            if let Err(err) = db
                .query("DELETE refresh_token WHERE expires_at < time::now();")
                .await
            {
                error!("[Cleaner] Failed to delete expired tokens: {err}");
            } else {
                info!("[Cleaner] Expired refresh_tokens cleaned");
            }
        }
    });
}

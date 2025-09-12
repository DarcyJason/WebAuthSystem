use ntex::web;

use crate::{
    config::Config,
    database::{client::DBClient, init::init_surrealdb},
    errors::app_error::AppResult,
    routes::api_routes,
    state::AppState,
};

pub async fn run() -> AppResult<()> {
    let config = Config::init()?;
    let surrealdb = init_surrealdb(config.clone()).await?;
    let db_client = DBClient::new(surrealdb);
    let app_state = AppState::new(config, db_client);
    web::HttpServer::new(move || {
        web::App::new()
            .state(app_state.clone())
            .configure(api_routes)
    })
    .bind(("127.0.0.1", 7878))?
    .run()
    .await?;
    Ok(())
}

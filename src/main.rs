use std::sync::Arc;

use quests_tracker::{config::config_loader, infrastructure::{axum_http::http_serve::start, postgres::postgres_connection}};

#[tokio::main]
async fn main() {
    // สำหรับสร้าง log
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            tracing::error!("Fail to load environment: {}", e);
            std::process::exit(1); // หยุดการำทงานของ application
        }
    };
    tracing::info!("Env has been loaded: {:?}", dotenvy_env);

    let postgres_pool = match postgres_connection::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Fail to connect to database: {}", e);
            std::process::exit(1);
        }
    };
    tracing::info!("Database connection successful");

    start(Arc::new(dotenvy_env), Arc::new(postgres_pool)).await.expect("Fail to start server")
}

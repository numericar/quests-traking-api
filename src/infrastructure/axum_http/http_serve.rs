use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{http::Method, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::{cors::{Any, CorsLayer}, limit::RequestBodyLimitLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

use crate::{config::config_model::DotEnvyConfig, infrastructure::postgres::postgres_connection::PgPoolSquad};

use super::default_router;

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> anyhow::Result<()> {
    let app: Router = Router::new()
        .fallback(default_router::not_found) // สำหรับกรณีที่หา route ไม่เจอเลย
        .route("/api/health-check", get(default_router::health_check))
        .layer(TimeoutLayer::new(Duration::from_secs(config.server.timeout))) // ใช้่สำหรับเพิ่ม middleware
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?
        ))
        .layer(CorsLayer::new().allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE
        ])
        .allow_origin(Any))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Server os running on port {}", config.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?; // เมื่อมีการสั่งหยุดทำงาน มันจะค่อย ๆ ทำงานที่เหลือยู่ให้เสร็จ แล้วค่อยปิด

    anyhow::Ok(())
}

async fn shutdown_signal() -> () {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install ctrl c signal handler")
    };

    let termimate = std::future::pending();

    tokio::select! {
        _ = ctrl_c => info!("Ctrl + C shut down"),
        _ = termimate => info!("termimate shut down"),
    }
}
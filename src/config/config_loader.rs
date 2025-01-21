use anyhow::{Ok, Result};
use super::config_model::{Database, DotEnvyConfig, Server};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok(); // โหลดข้อมูล env

    let server: Server = Server {
        port: std::env::var("PORT").expect("Server port is invalid").parse()?,
        body_limit: std::env::var("BODY_LIMIT").expect("Server port is invalid").parse()?,
        timeout: std::env::var("TIMEOUT").expect("Server port is invalid").parse()?
    };

    let database: Database = Database {
        url: std::env::var("DATABASE_URL").expect("Database url not found")
    };

    return Ok(DotEnvyConfig { server, database });
}
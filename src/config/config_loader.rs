use anyhow::{Ok, Result};
use super::{config_model::{AdventureSecert, Database, DotEnvyConfig, GuildCommanderSecert, Server}, stage::Stage};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok(); // โหลดข้อมูล env

    let server: Server = Server {
        port: std::env::var("SERVER_PORT").expect("SERVER_PORT is invalid").parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT").expect("SERVER_BODY_LIMIT is invalid").parse()?,
        timeout: std::env::var("SERVER_TIMEOUT").expect("SERVER_TIMEOUT is invalid").parse()?
    };

    let database: Database = Database {
        url: std::env::var("DATABASE_URL").expect("Database url not found")
    };

    return Ok(DotEnvyConfig { server, database });
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str: String = std::env::var("STAGE").unwrap_or_default().to_string();

    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_advanturer_secert_env() -> anyhow::Result<AdventureSecert> {
    dotenvy::dotenv().ok();

    Ok(AdventureSecert {
        secert: std::env::var("JWT_ADVENTURER_SECERT").expect("JWT_ADVENTURER_SECERT not found"),
        refresh_token: std::env::var("JWT_ADVENTURER_REFRESH_SECERT").expect("JWT_ADVENTURER_REFRESH_SECERT not found")
    })
}

pub fn get_guild_commander_secert_env() -> anyhow::Result<GuildCommanderSecert> {
    dotenvy::dotenv().ok();

    Ok(GuildCommanderSecert {
        secert: std::env::var("JWT_GUILD_COMMANDER_SECERT").expect("JWT_GUILD_COMMANDER_SECERT not found"),
        refresh_token: std::env::var("JWT_GUILD_COMMANDER_SECERT").expect("JWT_GUILD_COMMANDER_SECERT not found")
    })
}
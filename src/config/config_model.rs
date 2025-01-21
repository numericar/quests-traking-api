#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub server: Server,
    pub database: Database
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub timeout: u64,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String
}

#[derive(Debug, Clone)]
pub struct AdventureSecert {
    pub secert: String,
    pub refresh_token: String
}

#[derive(Debug, Clone)]
pub struct GuildCommanderSecert {
    pub secert: String,
    pub refresh_token: String
}
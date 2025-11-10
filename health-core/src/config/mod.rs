mod database;
mod file;
mod helpers;
mod http;

pub use database::DbConfig;
pub use file::{parse_config_file, parse_config_str, CheckConfig};
pub use http::HttpConfig;

pub const DEFAULT_HTTP_URL: &str = "http://localhost:12008";
pub const DEFAULT_HTTP_TIMEOUT_MS: u64 = 5_000;
pub const DEFAULT_DB_HOST: &str = "localhost";
pub const DEFAULT_DB_PORT: u16 = 5432;
pub const DEFAULT_DB_TIMEOUT_MS: u64 = 3_000;

#[derive(Clone)]
pub struct Config {
    http: HttpConfig,
    database: DbConfig,
}

impl Config {
    pub fn load() -> Self {
        Self {
            http: HttpConfig::from_env(),
            database: DbConfig::from_env(),
        }
    }

    pub fn http(&self) -> &HttpConfig {
        &self.http
    }

    pub fn database(&self) -> &DbConfig {
        &self.database
    }
}

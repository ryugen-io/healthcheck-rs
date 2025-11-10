use std::time::Duration;

use super::{
    DEFAULT_DB_HOST, DEFAULT_DB_PORT, DEFAULT_DB_TIMEOUT_MS,
    helpers::{env_or_default, parse_env_u16, parse_env_u64},
};

#[derive(Clone)]
pub struct DbConfig {
    host: String,
    port: u16,
    timeout: Duration,
}

impl DbConfig {
    pub fn new(host: String, port: u16, timeout: Duration) -> Self {
        Self {
            host,
            port,
            timeout,
        }
    }

    pub fn from_env() -> Self {
        let host = env_or_default("POSTGRES_HOST", DEFAULT_DB_HOST);
        let port = parse_env_u16("POSTGRES_PORT", DEFAULT_DB_PORT);
        let timeout =
            Duration::from_millis(parse_env_u64("POSTGRES_TIMEOUT_MS", DEFAULT_DB_TIMEOUT_MS));

        Self {
            host,
            port,
            timeout,
        }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}

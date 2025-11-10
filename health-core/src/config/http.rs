use std::time::Duration;

use super::{
    DEFAULT_HTTP_TIMEOUT_MS, DEFAULT_HTTP_URL,
    helpers::{env_or_default, parse_env_u64},
};

#[derive(Clone)]
pub struct HttpConfig {
    url: String,
    timeout: Duration,
}

impl HttpConfig {
    pub fn new(url: String, timeout: Duration) -> Self {
        Self { url, timeout }
    }

    pub fn from_env() -> Self {
        let url = env_or_default("METAMCP_URL", DEFAULT_HTTP_URL);
        let timeout = Duration::from_millis(parse_env_u64(
            "METAMCP_HTTP_TIMEOUT_MS",
            DEFAULT_HTTP_TIMEOUT_MS,
        ));

        Self { url, timeout }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}

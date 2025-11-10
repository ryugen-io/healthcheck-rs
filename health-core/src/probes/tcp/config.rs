use std::collections::HashMap;
use std::time::Duration;

/// TCP check configuration
#[derive(Debug, Clone)]
pub struct TcpConfig {
    host: String,
    port: u16,
    timeout: Duration,
}

impl TcpConfig {
    /// Parse from config parameters
    pub fn from_params(params: &HashMap<String, String>) -> Result<Self, String> {
        let host = params
            .get("host")
            .cloned()
            .unwrap_or_else(|| "localhost".to_string());

        let port = params
            .get("port")
            .ok_or("missing required param: port")?
            .parse::<u16>()
            .map_err(|e| format!("invalid port: {e}"))?;

        let timeout_ms = params
            .get("timeout_ms")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(2000);

        Ok(Self {
            host,
            port,
            timeout: Duration::from_millis(timeout_ms),
        })
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

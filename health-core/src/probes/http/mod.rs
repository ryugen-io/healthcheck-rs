use crate::config::HttpConfig;
use crate::registry::HealthCheck;

use super::{elapsed_ms, ProbeResult};
use log::{error, info};
use std::collections::HashMap;
use std::time::{Duration, Instant};

mod transport;
mod url;

use transport::perform_request;
use url::parse_http_url;

/// HTTP endpoint check
pub struct HttpCheck {
    config: HttpConfig,
}

impl HttpCheck {
    /// Create from config parameters
    pub fn from_params(params: &HashMap<String, String>) -> Result<Box<dyn HealthCheck>, String> {
        let url = params
            .get("url")
            .ok_or("missing required param: url")?
            .clone();

        let timeout_ms = params
            .get("timeout_ms")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(5000);

        Ok(Box::new(Self {
            config: HttpConfig::new(url, Duration::from_millis(timeout_ms)),
        }))
    }
}

impl HealthCheck for HttpCheck {
    fn check(&self) -> ProbeResult {
        check_http(&self.config)
    }

    fn name(&self) -> &str {
        "http"
    }
}

pub fn check_http(config: &HttpConfig) -> ProbeResult {
    let start = Instant::now();
    match http_latency_ms(config) {
        Ok(latency) => {
            info!("HTTP probe succeeded in {latency}ms");
            ProbeResult::success(latency)
        }
        Err(err) => {
            let latency = elapsed_ms(start);
            error!("HTTP probe error: {err}");
            ProbeResult::failure(latency, err)
        }
    }
}

fn http_latency_ms(config: &HttpConfig) -> Result<u64, String> {
    let target = parse_http_url(config.url()).map_err(|err| format!("invalid URL: {err}"))?;
    let start = Instant::now();
    perform_request(&target, config.timeout())?;
    Ok(elapsed_ms(start))
}

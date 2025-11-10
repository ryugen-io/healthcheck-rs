mod config;

pub use config::TcpConfig;

use crate::probes::{elapsed_ms, ProbeResult};
use crate::registry::HealthCheck;
use log::{error, info};
use std::collections::HashMap;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Instant;

/// TCP port connectivity check
pub struct TcpCheck {
    config: TcpConfig,
}

impl TcpCheck {
    /// Create from config parameters
    pub fn from_params(params: &HashMap<String, String>) -> Result<Box<dyn HealthCheck>, String> {
        let config = TcpConfig::from_params(params)?;
        Ok(Box::new(Self { config }))
    }
}

impl HealthCheck for TcpCheck {
    fn check(&self) -> ProbeResult {
        let start = Instant::now();
        let addr = format!("{}:{}", self.config.host(), self.config.port());

        match perform_tcp_check(&addr, self.config.timeout()) {
            Ok(_) => {
                let latency = elapsed_ms(start);
                info!("TCP check succeeded for {addr} in {latency}ms");
                ProbeResult::success(latency)
            }
            Err(err) => {
                let latency = elapsed_ms(start);
                error!("TCP check failed for {addr}: {err}");
                ProbeResult::failure(latency, err)
            }
        }
    }

    fn name(&self) -> &str {
        "tcp"
    }
}

fn perform_tcp_check(addr: &str, timeout: std::time::Duration) -> Result<(), String> {
    let addrs: Vec<_> = addr
        .to_socket_addrs()
        .map_err(|e| format!("failed to resolve address: {e}"))?
        .collect();

    if addrs.is_empty() {
        return Err("no addresses resolved".to_string());
    }

    let mut last_err = None;

    for sock_addr in addrs {
        match TcpStream::connect_timeout(&sock_addr, timeout) {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_err = Some(e.to_string());
            }
        }
    }

    Err(last_err.unwrap_or_else(|| "connection failed".to_string()))
}

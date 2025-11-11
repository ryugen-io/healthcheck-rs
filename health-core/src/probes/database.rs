use crate::registry::HealthCheck;

use super::{ProbeResult, elapsed_ms};
use log::{error, info};
use std::collections::HashMap;
use std::time::Instant;

/// PostgreSQL database check
pub struct DatabaseCheck {
    conn_str: String,
    timeout_ms: u64,
}

impl DatabaseCheck {
    /// Create from config parameters
    pub fn from_params(params: &HashMap<String, String>) -> Result<Box<dyn HealthCheck>, String> {
        // Try conn_str first (full connection string)
        if let Some(conn_str) = params.get("conn_str") {
            let timeout_ms = params
                .get("timeout_ms")
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(3000);

            return Ok(Box::new(Self {
                conn_str: conn_str.clone(),
                timeout_ms,
            }));
        }

        // Fallback: build from host/port/user/password/db
        let host = params
            .get("host")
            .cloned()
            .unwrap_or_else(|| "localhost".to_string());

        let port = params
            .get("port")
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(5432);

        let user = params
            .get("user")
            .cloned()
            .unwrap_or_else(|| "postgres".to_string());

        let password = params.get("password").cloned().unwrap_or_default();

        let dbname = params
            .get("dbname")
            .cloned()
            .unwrap_or_else(|| "postgres".to_string());

        let timeout_ms = params
            .get("timeout_ms")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(3000);

        // Pre-allocate capacity for connection string to avoid reallocations
        let mut conn_str = String::with_capacity(
            host.len() + user.len() + dbname.len() + password.len() + 50
        );
        
        conn_str.push_str("host=");
        conn_str.push_str(&host);
        conn_str.push_str(" port=");
        conn_str.push_str(&port.to_string());
        conn_str.push_str(" user=");
        conn_str.push_str(&user);
        
        if !password.is_empty() {
            conn_str.push_str(" password=");
            conn_str.push_str(&password);
        }
        
        conn_str.push_str(" dbname=");
        conn_str.push_str(&dbname);

        Ok(Box::new(Self {
            conn_str,
            timeout_ms,
        }))
    }
}

impl HealthCheck for DatabaseCheck {
    fn check(&self) -> ProbeResult {
        check_database(&self.conn_str, self.timeout_ms)
    }

    fn name(&self) -> &str {
        "database"
    }
}

fn check_database(conn_str: &str, timeout_ms: u64) -> ProbeResult {
    let start = Instant::now();

    // Parse connection string and set timeout
    let config = match conn_str.parse::<postgres::Config>() {
        Ok(mut cfg) => {
            cfg.connect_timeout(std::time::Duration::from_millis(timeout_ms));
            cfg
        }
        Err(err) => {
            let latency = elapsed_ms(start);
            let err_msg = format!("Invalid connection string: {}", err);
            error!("Database probe failed: {}", err_msg);
            return ProbeResult::failure(latency, err_msg);
        }
    };

    match config.connect(postgres::NoTls) {
        Ok(mut client) => {
            // Execute simple query to verify connection works
            match client.simple_query("SELECT 1") {
                Ok(_) => {
                    let latency = elapsed_ms(start);
                    info!("Database probe succeeded in {}ms", latency);
                    ProbeResult::success(latency)
                }
                Err(err) => {
                    let latency = elapsed_ms(start);
                    let err_msg = err.to_string();
                    error!("Database query failed: {}", err_msg);
                    ProbeResult::failure(latency, err_msg)
                }
            }
        }
        Err(err) => {
            let latency = elapsed_ms(start);
            let err_msg = err.to_string();
            error!("Database connection failed: {}", err_msg);
            ProbeResult::failure(latency, err_msg)
        }
    }
}

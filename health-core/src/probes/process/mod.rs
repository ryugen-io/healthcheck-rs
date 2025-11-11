mod config;

pub use config::ProcessConfig;

use crate::probes::{ProbeResult, elapsed_ms};
use crate::registry::HealthCheck;
use log::{error, info};
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

/// Process running check
pub struct ProcessCheck {
    config: ProcessConfig,
}

impl ProcessCheck {
    /// Create from config parameters
    pub fn from_params(params: &HashMap<String, String>) -> Result<Box<dyn HealthCheck>, String> {
        let config = ProcessConfig::from_params(params)?;
        Ok(Box::new(Self { config }))
    }
}

impl HealthCheck for ProcessCheck {
    fn check(&self) -> ProbeResult {
        let start = Instant::now();
        let process_name = self.config.name();

        match is_process_running(process_name) {
            Ok(true) => {
                let latency = elapsed_ms(start);
                info!("Process check succeeded for '{process_name}' in {latency}ms");
                ProbeResult::success(latency)
            }
            Ok(false) => {
                let latency = elapsed_ms(start);
                error!("Process '{process_name}' not running");
                ProbeResult::failure(latency, format!("process '{process_name}' not found"))
            }
            Err(err) => {
                let latency = elapsed_ms(start);
                error!("Process check error for '{process_name}': {err}");
                ProbeResult::failure(latency, err)
            }
        }
    }

    fn name(&self) -> &str {
        "process"
    }
}

/// Check if a process is running by reading /proc
fn is_process_running(process_name: &str) -> Result<bool, String> {
    let proc_dir = fs::read_dir("/proc").map_err(|e| format!("failed to read /proc: {e}"))?;

    for entry in proc_dir {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        // Only check numeric directories (PIDs)
        let file_name = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(_) => continue,
        };

        if !file_name.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }

        // Early return optimization: exit as soon as we find the process
        let cmdline_path = format!("/proc/{}/comm", file_name);
        if let Ok(comm) = fs::read_to_string(&cmdline_path)
            && comm.trim() == process_name
        {
            return Ok(true);
        }
    }

    Ok(false)
}

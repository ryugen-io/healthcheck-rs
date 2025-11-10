use log::{info, warn};
use std::env;

pub fn env_or_default(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(val) if !val.trim().is_empty() => val,
        _ => {
            if env::var(key).is_err() {
                info!("Using default for {key}: {default}");
            } else {
                warn!("Environment variable {key} is empty; falling back to default");
            }
            default.to_string()
        }
    }
}

pub fn parse_env_u64(key: &str, default: u64) -> u64 {
    match env::var(key) {
        Ok(val) => match val.parse::<u64>() {
            Ok(parsed) => parsed,
            Err(err) => {
                warn!("Failed to parse {key}={val}: {err}; defaulting to {default}");
                default
            }
        },
        Err(_) => default,
    }
}

pub fn parse_env_u16(key: &str, default: u16) -> u16 {
    match env::var(key) {
        Ok(val) => match val.parse::<u16>() {
            Ok(parsed) => parsed,
            Err(err) => {
                warn!("Failed to parse {key}={val}: {err}; defaulting to {default}");
                default
            }
        },
        Err(_) => default,
    }
}

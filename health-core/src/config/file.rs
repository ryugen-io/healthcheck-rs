use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Simple config format:
// # comment
// type:param1=value1,param2=value2
// 
// Example:
// tcp:host=localhost,port=21116,timeout_ms=2000
// http:url=http://localhost:12008

/// Individual check configuration
#[derive(Debug, Clone)]
pub struct CheckConfig {
    pub check_type: String,
    pub params: HashMap<String, String>,
}

/// Parse a config file into check configurations
pub fn parse_config_file<P: AsRef<Path>>(path: P) -> Result<Vec<CheckConfig>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("failed to read config: {e}"))?;
    parse_config_str(&content)
}

/// Parse config from string
pub fn parse_config_str(content: &str) -> Result<Vec<CheckConfig>, String> {
    let mut checks = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Split type:params
        let (check_type, params_str) = line
            .split_once(':')
            .ok_or_else(|| format!("line {}: missing ':' separator", line_num + 1))?;

        let params = parse_params(params_str, line_num + 1)?;

        checks.push(CheckConfig {
            check_type: check_type.trim().to_string(),
            params,
        });
    }

    Ok(checks)
}

/// Parse param1=value1,param2=value2 into HashMap
fn parse_params(params_str: &str, line_num: usize) -> Result<HashMap<String, String>, String> {
    let mut params = HashMap::new();

    for pair in params_str.split(',') {
        let pair = pair.trim();
        if pair.is_empty() {
            continue;
        }

        let (key, value) = pair
            .split_once('=')
            .ok_or_else(|| format!("line {line_num}: param missing '=' in '{pair}'"))?;

        params.insert(key.trim().to_string(), value.trim().to_string());
    }

    Ok(params)
}

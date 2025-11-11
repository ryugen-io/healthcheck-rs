use healthcheck_core::{
    config::parse_config_file,
    probes::{database::DatabaseCheck, http::HttpCheck, process::ProcessCheck, tcp::TcpCheck},
    registry::CheckRegistry,
};
use log::{error, info};
use std::env;

fn init_logger() {
    if env::var("RUST_LOG").is_err() {
        // SAFETY: this executes during start-up before any other threads spawn.
        unsafe { env::set_var("RUST_LOG", "info") };
    }

    let _ = env_logger::builder().format_timestamp_millis().try_init();
}

fn build_registry() -> CheckRegistry {
    let mut registry = CheckRegistry::new();

    registry.register("tcp", TcpCheck::from_params);
    registry.register("http", HttpCheck::from_params);
    registry.register("database", DatabaseCheck::from_params);
    registry.register("process", ProcessCheck::from_params);

    registry
}

fn main() {
    init_logger();

    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "healthcheck.config".to_string());

    info!("Loading healthcheck config from: {config_path}");

    let check_configs = match parse_config_file(&config_path) {
        Ok(configs) => configs,
        Err(e) => {
            error!("Failed to parse config: {e}");
            print_error_json(&format!("config parse error: {e}"));
            std::process::exit(2);
        }
    };

    if check_configs.is_empty() {
        error!("No checks configured");
        print_error_json("no checks configured");
        std::process::exit(2);
    }

    info!("Running {} health checks", check_configs.len());

    let registry = build_registry();
    let mut results = Vec::with_capacity(check_configs.len());
    let mut overall_ok = true;

    for check_config in check_configs {
        let check = match registry.create_check(&check_config.check_type, &check_config.params) {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to create check '{}': {e}", check_config.check_type);
                results.push(CheckResult {
                    check_type: check_config.check_type.clone(),
                    ok: false,
                    latency_ms: 0,
                    error: Some(format!("check creation failed: {e}")),
                });
                overall_ok = false;
                continue;
            }
        };

        let result = check.check();

        if !result.ok {
            overall_ok = false;
        }

        results.push(CheckResult {
            check_type: check_config.check_type.clone(),
            ok: result.ok,
            latency_ms: result.latency_ms,
            error: result.error,
        });
    }

    print_results(&results, overall_ok);

    if !overall_ok {
        std::process::exit(1);
    }
}

#[derive(Debug)]
struct CheckResult {
    check_type: String,
    ok: bool,
    latency_ms: u64,
    error: Option<String>,
}

fn print_results(results: &[CheckResult], overall: bool) {
    println!("{{");
    println!("  \"overall\": {},", if overall { "true" } else { "false" });
    println!("  \"checks\": [");

    for (i, result) in results.iter().enumerate() {
        let comma = if i < results.len() - 1 { "," } else { "" };
        println!("    {{");
        println!("      \"type\": \"{}\",", result.check_type);
        println!(
            "      \"ok\": {},",
            if result.ok { "true" } else { "false" }
        );
        println!("      \"latency_ms\": {},", result.latency_ms);

        if let Some(err) = &result.error {
            let escaped = escape_json_string(err);
            println!("      \"error\": \"{}\"", escaped);
        } else {
            println!("      \"error\": null");
        }

        println!("    }}{}", comma);
    }

    println!("  ]");
    println!("}}");
}

fn print_error_json(message: &str) {
    let escaped = escape_json_string(message);
    println!("{{");
    println!("  \"overall\": false,");
    println!("  \"error\": \"{}\"", escaped);
    println!("}}");
}

/// Escape special characters in JSON strings more efficiently
fn escape_json_string(s: &str) -> String {
    // Pre-allocate with some extra capacity for escape sequences
    let mut result = String::with_capacity(s.len() + 16);
    
    for ch in s.chars() {
        match ch {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(ch),
        }
    }
    
    result
}

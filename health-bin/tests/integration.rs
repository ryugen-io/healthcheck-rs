use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_healthcheck_bin() -> PathBuf {
    env::var("CARGO_BIN_EXE_healthcheckrs")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.pop();
            path.push("target");
            path.push("debug");
            path.push("healthcheckrs");
            path
        })
}

#[test]
fn healthcheck_exits_on_invalid_config() {
    let output = Command::new(get_healthcheck_bin())
        .arg("/nonexistent/path/config.conf")
        .output()
        .expect("failed to execute healthcheck");

    assert!(!output.status.success());
}

#[test]
fn healthcheck_uses_default_config() {
    let output = Command::new(get_healthcheck_bin())
        .output()
        .expect("failed to execute healthcheck");

    // Uses default "healthcheck.config" which doesn't exist, so exits with error
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should output JSON error
    assert!(stdout.contains("\"overall\": false"));
}

#[test]
fn healthcheck_parses_valid_config() {
    let mut config_path = env::temp_dir();
    config_path.push("test_config.conf");

    let config_content = "tcp:host=127.0.0.1,port=22,timeout_ms=1000\n";
    fs::write(&config_path, config_content).expect("failed to write config");

    let output = Command::new(get_healthcheck_bin())
        .arg(&config_path)
        .output()
        .expect("failed to execute healthcheck");

    fs::remove_file(&config_path).ok();

    assert!(output.status.success());
}

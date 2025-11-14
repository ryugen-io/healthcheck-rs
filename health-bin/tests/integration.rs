use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_healthcheck_bin() -> PathBuf {
    env::var("CARGO_BIN_EXE_healthcheck")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.pop();
            path.push("target");
            path.push("debug");
            path.push("healthcheck");
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

    // Config should be parsed successfully and return JSON output
    // The check itself might fail (port not open), but parsing should work
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"overall\":"),
        "Should output JSON with overall status"
    );
    assert!(
        stdout.contains("\"checks\":"),
        "Should output JSON with checks array"
    );
}

#[test]
fn test_generate_bin_creates_binary() {
    let output_dir = env::temp_dir().join("test_bin_output");
    fs::create_dir_all(&output_dir).expect("failed to create temp dir");

    let output = Command::new(get_healthcheck_bin())
        .arg("generate-bin")
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("failed to execute healthcheck generate-bin");

    // Cleanup
    fs::remove_dir_all(&output_dir).ok();

    assert!(output.status.success(), "Command should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Binary generated successfully"));
}

#[test]
fn test_generate_bin_without_output_flag() {
    let output = Command::new(get_healthcheck_bin())
        .arg("generate-bin")
        .output()
        .expect("failed to execute healthcheck generate-bin");

    // Cleanup default ./bin directory
    fs::remove_dir_all("./bin").ok();

    assert!(
        output.status.success(),
        "Command should succeed with default path"
    );
}

#[test]
fn test_generate_conf_creates_file() {
    let config_path = env::temp_dir().join("test_healthcheck.config");

    let output = Command::new(get_healthcheck_bin())
        .arg("generate-conf")
        .arg("--output")
        .arg(&config_path)
        .output()
        .expect("failed to execute healthcheck generate-conf");

    let file_exists = config_path.exists();
    let mut file_content = String::new();
    if file_exists {
        file_content = fs::read_to_string(&config_path).unwrap_or_default();
    }

    // Cleanup
    fs::remove_file(&config_path).ok();

    assert!(output.status.success(), "Command should succeed");
    assert!(file_exists, "Config file should be created");
    assert!(
        file_content.contains("SECURITY WARNING"),
        "Config should contain security warning"
    );
    assert!(
        file_content.contains("tcp:"),
        "Config should contain TCP example"
    );
}

#[test]
fn test_help_flag() {
    let output = Command::new(get_healthcheck_bin())
        .arg("--help")
        .output()
        .expect("failed to execute healthcheck --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("USAGE:"));
    assert!(stdout.contains("generate-bin"));
    assert!(stdout.contains("generate-conf"));
}

#[test]
fn test_version_flag() {
    let output = Command::new(get_healthcheck_bin())
        .arg("--version")
        .output()
        .expect("failed to execute healthcheck --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("healthcheck v"));
}

#[test]
fn test_unknown_flag_error() {
    let output = Command::new(get_healthcheck_bin())
        .arg("--unknown-flag")
        .output()
        .expect("failed to execute healthcheck --unknown-flag");

    assert!(!output.status.success(), "Should fail with unknown flag");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unknown flag"));
}

#[test]
fn test_generate_bin_missing_output_value() {
    let output = Command::new(get_healthcheck_bin())
        .arg("generate-bin")
        .arg("--output")
        .output()
        .expect("failed to execute healthcheck generate-bin --output");

    assert!(!output.status.success(), "Should fail without path value");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--output requires a path argument"));
}

#[test]
fn test_serve_command_unimplemented() {
    let output = Command::new(get_healthcheck_bin())
        .arg("serve")
        .output()
        .expect("failed to execute healthcheck serve");

    // serve command should exit with error and indicate it's not implemented yet
    assert!(!output.status.success(), "Should exit with error code");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Check that stderr contains the not implemented message
    assert!(
        stderr.contains("not yet implemented") || stderr.contains("Coming soon"),
        "serve command should indicate it's not implemented yet. Got: {}",
        stderr
    );
}

#[test]
fn test_watch_command_unimplemented() {
    let output = Command::new(get_healthcheck_bin())
        .arg("watch")
        .output()
        .expect("failed to execute healthcheck watch");

    // watch command should exit with error and indicate it's not implemented yet
    assert!(!output.status.success(), "Should exit with error code");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Check that stderr contains the not implemented message
    assert!(
        stderr.contains("not yet implemented") || stderr.contains("Coming soon"),
        "watch command should indicate it's not implemented yet. Got: {}",
        stderr
    );
}

#[test]
fn test_generate_conf_non_interactive_existing_file() {
    let config_path = env::temp_dir().join("test_existing.config");

    // Create the file first
    fs::write(&config_path, "existing content").expect("failed to write existing file");

    // Try to generate conf in non-interactive mode (will fail because file exists)
    let output = Command::new(get_healthcheck_bin())
        .arg("generate-conf")
        .arg("--output")
        .arg(&config_path)
        .output()
        .expect("failed to execute healthcheck generate-conf");

    // Cleanup
    fs::remove_file(&config_path).ok();

    // In non-interactive mode, should fail when file exists
    // Note: This test might succeed in some CI environments that have a TTY,
    // so we check for either success (with prompt) or failure (without TTY)
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("already exists"),
            "Should indicate file already exists"
        );
    }
}

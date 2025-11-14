use super::*;
use std::env;

#[test]
fn test_validate_output_path_relative() {
    // Relative paths in current directory should work
    let result = validate_output_path("./test_output");
    assert!(
        result.is_ok(),
        "Should allow relative path in current dir: {:?}",
        result
    );

    let result = validate_output_path("bin");
    assert!(
        result.is_ok(),
        "Should allow simple relative path: {:?}",
        result
    );
}

#[test]
fn test_validate_output_path_file() {
    // File paths should work
    let result = validate_output_path("./config.txt");
    assert!(result.is_ok(), "Should allow file path");
}

#[test]
fn test_validate_output_path_extensionless_file() {
    // Extensionless files should be treated as files
    let result = validate_output_path("./Makefile");
    assert!(result.is_ok(), "Should allow extensionless file");

    let result = validate_output_path("./healthcheck.config");
    assert!(result.is_ok(), "Should allow config without extension");
}

#[test]
fn test_validate_output_path_directory_with_slash() {
    // Paths ending with / should be treated as directories
    let result = validate_output_path("./test_dir/");
    assert!(result.is_ok(), "Should allow directory with trailing slash");
}

#[test]
fn test_validate_output_path_nonexistent_parent() {
    // Should fail if parent directory doesn't exist
    let result = validate_output_path("./nonexistent_dir/subdir/output");
    assert!(
        result.is_err(),
        "Should reject path with nonexistent parent"
    );
}

#[test]
#[cfg(unix)]
fn test_validate_output_path_rejects_system_dirs_unix() {
    // Direct system directories should be rejected
    assert!(validate_output_path("/etc").is_err(), "Should reject /etc");
    assert!(validate_output_path("/sys").is_err(), "Should reject /sys");
    assert!(
        validate_output_path("/proc").is_err(),
        "Should reject /proc"
    );
    assert!(validate_output_path("/dev").is_err(), "Should reject /dev");
    assert!(validate_output_path("/bin").is_err(), "Should reject /bin");
    assert!(
        validate_output_path("/sbin").is_err(),
        "Should reject /sbin"
    );
}

#[test]
#[cfg(windows)]
fn test_validate_output_path_rejects_system_dirs_windows() {
    assert!(
        validate_output_path("C:\\Windows").is_err(),
        "Should reject C:\\Windows"
    );
    assert!(
        validate_output_path("C:\\Program Files").is_err(),
        "Should reject C:\\Program Files"
    );
}

#[test]
fn test_validate_output_path_traversal_to_system() {
    // If traversal leads to system directory, it should be rejected
    // This test creates a path that would traverse to /etc or similar
    let cwd = env::current_dir().expect("Failed to get current dir");

    // Try to construct a path that goes up enough levels to reach /etc
    #[cfg(unix)]
    {
        // Build a path like ../../../etc (enough ../ to reach root)
        let depth = cwd.components().count();
        let mut traversal = String::new();
        for _ in 0..depth + 1 {
            traversal.push_str("../");
        }
        traversal.push_str("etc");

        let result = validate_output_path(&traversal);
        assert!(
            result.is_err(),
            "Should reject traversal to /etc: {:?}",
            result
        );
    }
}

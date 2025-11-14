use super::*;

#[test]
fn test_validate_output_path_directory_with_dots() {
    // Create a test directory with dots in the name
    let test_dir = "./test.config.dir";
    std::fs::create_dir_all(test_dir).ok();

    // Test that a directory with dots is handled correctly
    let result = validate_output_path(test_dir);
    assert!(
        result.is_ok(),
        "Should allow directory with dots: {:?}",
        result
    );

    // Clean up
    std::fs::remove_dir(test_dir).ok();
}

#[test]
fn test_validate_output_path_file_in_dotted_directory() {
    // Create a test directory with dots
    let test_dir = "./test.backup.dir";
    std::fs::create_dir_all(test_dir).ok();

    // Test file path within dotted directory
    let file_path = format!("{}/config.txt", test_dir);
    let result = validate_output_path(&file_path);
    assert!(
        result.is_ok(),
        "Should allow file in dotted directory: {:?}",
        result
    );

    // Clean up
    std::fs::remove_dir(test_dir).ok();
}

#[test]
#[cfg(unix)]
fn test_validate_output_path_symlink() {
    use std::os::unix::fs::symlink;

    // Create a test directory and a symlink to it
    let test_dir = "./test_real_dir";
    let symlink_path = "./test_symlink_dir";

    std::fs::create_dir_all(test_dir).ok();
    symlink(test_dir, symlink_path).ok();

    // Test that symlinks are resolved correctly
    let result = validate_output_path(symlink_path);
    assert!(result.is_ok(), "Should resolve symlink: {:?}", result);

    // Verify it resolves to the real path
    if let Ok(canonical) = result {
        assert!(
            canonical.to_string_lossy().contains("test_real_dir"),
            "Symlink should resolve to real directory"
        );
    }

    // Clean up
    std::fs::remove_file(symlink_path).ok();
    std::fs::remove_dir(test_dir).ok();
}

#[test]
fn test_validate_output_path_nonexistent_directory_with_dots() {
    // Test a non-existent directory with dots (should be treated as file without trailing slash)
    let test_path = "./test.output.dir";

    // Ensure it doesn't exist
    std::fs::remove_dir_all(test_path).ok();

    // Without trailing slash, will be treated as a file, so parent must exist
    let result = validate_output_path(test_path);
    // Current directory exists, so this should succeed
    assert!(
        result.is_ok(),
        "Should handle non-existent path with dots: {:?}",
        result
    );

    // With trailing slash, should be treated as directory
    let test_path_with_slash = "./test.output.dir/";
    let result = validate_output_path(test_path_with_slash);
    assert!(
        result.is_ok(),
        "Should handle non-existent directory path with dots and slash: {:?}",
        result
    );
}

#[test]
fn test_validate_output_path_hidden_file() {
    // Hidden files (starting with dot) should work
    let result = validate_output_path("./.hidden_config");
    assert!(result.is_ok(), "Should allow hidden files");
}

#[test]
fn test_validate_output_path_multiple_dots() {
    // Files with multiple dots in extension
    let result = validate_output_path("./archive.tar.gz");
    assert!(result.is_ok(), "Should allow files with multiple dots");
}

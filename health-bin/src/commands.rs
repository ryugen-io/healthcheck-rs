use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Validate and safely resolve output path to prevent path traversal attacks
/// Uses canonicalization to prevent TOCTOU vulnerabilities
fn validate_output_path(path: &str) -> Result<PathBuf, String> {
    let path_obj = Path::new(path);

    // For paths that don't exist yet, validate the parent directory
    let (dir_to_check, is_file) = if path_obj.extension().is_some() {
        // Looks like a file path, check parent directory
        (path_obj.parent().unwrap_or_else(|| Path::new(".")), true)
    } else {
        // Directory path, check it directly
        (path_obj, false)
    };

    // Check if the directory exists, or if its parent exists
    let parent_exists = dir_to_check.exists();
    if !parent_exists && dir_to_check != Path::new(".") && dir_to_check != Path::new("") {
        // Check if parent directory exists
        let parent = dir_to_check.parent().unwrap_or_else(|| Path::new("."));
        // Only require parent to exist if it's not the current directory
        if parent != Path::new(".") && parent != Path::new("") && !parent.exists() {
            return Err(format!(
                "Parent directory '{}' does not exist",
                parent.display()
            ));
        }
    }

    // Try to canonicalize the directory (resolves symlinks and ..)
    let canonical = if parent_exists {
        dir_to_check
            .canonicalize()
            .map_err(|e| format!("Failed to resolve path '{}': {}", path, e))?
    } else {
        // Directory doesn't exist, validate parent and construct path
        let parent = dir_to_check.parent();

        let parent_canonical = if let Some(p) = parent {
            if p == Path::new("") || p == Path::new(".") {
                // Parent is current directory
                env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?
            } else {
                // Parent is some other directory, canonicalize it
                p.canonicalize().map_err(|e| {
                    format!("Failed to resolve parent path '{}': {}", p.display(), e)
                })?
            }
        } else {
            // No parent, use current directory
            env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?
        };

        let dir_name = dir_to_check
            .file_name()
            .ok_or_else(|| "Invalid directory name".to_string())?;
        parent_canonical.join(dir_name)
    };

    // Check against system directories after canonicalization
    let canonical_str = canonical.to_string_lossy();

    #[cfg(unix)]
    {
        if canonical_str.starts_with("/etc")
            || canonical_str.starts_with("/sys")
            || canonical_str.starts_with("/proc")
            || canonical_str.starts_with("/dev")
            || canonical_str.starts_with("/boot")
        {
            return Err(format!(
                "Access to system directory '{}' is not allowed",
                canonical_str
            ));
        }
    }

    #[cfg(windows)]
    {
        let lower = canonical_str.to_lowercase();
        if lower.starts_with("c:\\windows") || lower.starts_with("c:\\program files") {
            return Err(format!(
                "Access to system directory '{}' is not allowed",
                canonical_str
            ));
        }
    }

    // Return the full path (including filename if it was a file path)
    if is_file {
        let filename = path_obj
            .file_name()
            .ok_or_else(|| "Invalid filename".to_string())?;
        Ok(canonical.join(filename))
    } else {
        Ok(canonical)
    }
}

fn get_platform_info() -> (&'static str, &'static str, &'static str) {
    let os = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "unknown"
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "amd64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else {
        "unknown"
    };

    let ext = if cfg!(target_os = "windows") {
        ".exe"
    } else {
        ""
    };

    (os, arch, ext)
}

pub fn generate_bin(output_dir: Option<String>) -> Result<(), String> {
    let (os, arch, ext) = get_platform_info();

    // Get current executable path
    let current_exe =
        env::current_exe().map_err(|e| format!("Failed to get current executable path: {}", e))?;

    // Determine and validate output directory
    let out_dir = output_dir.as_deref().unwrap_or("./bin");
    let validated_path = validate_output_path(out_dir)?;

    fs::create_dir_all(&validated_path)
        .map_err(|e| format!("Failed to create output directory '{}': {}", out_dir, e))?;

    // Generate platform-specific binary name
    let binary_name = format!("healthcheck-{}-{}{}", os, arch, ext);
    let output_path = validated_path.join(&binary_name);

    println!("Generating binary for deployment...");
    println!("  Platform: {}-{}", os, arch);
    println!("  Source:   {}", current_exe.display());
    println!("  Target:   {}", output_path.display());

    // Copy the binary
    fs::copy(&current_exe, &output_path).map_err(|e| format!("Failed to copy binary: {}", e))?;

    // Make it executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&output_path)
            .map_err(|e| format!("Failed to read file metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&output_path, perms)
            .map_err(|e| format!("Failed to set permissions: {}", e))?;
    }

    println!("Binary generated successfully!");
    println!();
    println!("Docker example:");
    println!("  COPY {} /usr/local/bin/healthcheck", binary_name);
    println!("  RUN chmod +x /usr/local/bin/healthcheck");

    Ok(())
}

pub fn generate_conf(output_path: Option<String>) -> Result<(), String> {
    let config_path = output_path.as_deref().unwrap_or("healthcheck.config");
    let validated_path = validate_output_path(config_path)?;

    let example_config = r#"# HealthCheck RS Configuration
# Format: check_type:param1=value1,param2=value2

# SECURITY WARNING:
# This config contains example credentials. DO NOT use these in production!
# Use environment variables for sensitive data:
#   - Set DB_PASSWORD env var and reference it in your application
#   - Use secrets management (Vault, AWS Secrets Manager, etc.)
#   - Never commit real credentials to version control

# TCP Port Checks
# Check if a TCP port is open and accepting connections
tcp:host=localhost,port=8080,timeout_ms=1000
tcp:host=localhost,port=5432,timeout_ms=1000

# HTTP Endpoint Checks
# Check if HTTP endpoints return 2xx or 3xx status codes
http:url=http://localhost:8080/health,timeout_ms=5000
http:url=http://localhost:3000/api/health,timeout_ms=3000

# Database Checks (PostgreSQL)
# SECURITY: Replace 'user:password' with environment variables in production!
# Example: postgresql://${DB_USER}:${DB_PASSWORD}@localhost:5432/dbname
database:conn_str=postgresql://user:CHANGE_ME@localhost:5432/dbname,timeout_ms=3000
# Or use individual parameters:
# database:host=localhost,port=5432,user=postgres,password=CHANGE_ME,dbname=mydb,timeout_ms=3000

# Process Checks (Linux only)
# Check if a process is running by name
process:name=nginx
process:name=postgres

# Multiple checks can be combined
# All checks run in parallel for fast results
"#;

    // Check if file already exists
    if validated_path.exists() {
        eprintln!(
            "Warning: File '{}' already exists and will be overwritten.",
            config_path
        );
        eprintln!("Press Ctrl+C to cancel, or Enter to continue...");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;
    }

    let mut file = fs::File::create(&validated_path)
        .map_err(|e| format!("Failed to create config file '{}': {}", config_path, e))?;

    file.write_all(example_config.as_bytes())
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    println!("Generated example configuration:");
    println!("  File: {}", config_path);
    println!();
    println!("Edit the file and run:");
    println!("  healthcheck {}", config_path);

    Ok(())
}

#[cfg(test)]
mod tests {
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
}

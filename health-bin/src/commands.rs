use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Validate and canonicalize output path to prevent path traversal attacks
fn validate_output_path(path: &str) -> Result<PathBuf, String> {
    // Reject obvious path traversal patterns
    if path.contains("..") {
        return Err("Path traversal detected: '..' is not allowed".to_string());
    }

    // Reject absolute paths to sensitive directories
    let path_obj = Path::new(path);
    if path_obj.is_absolute() {
        let path_str = path_obj.to_string_lossy();
        if path_str.starts_with("/etc")
            || path_str.starts_with("/sys")
            || path_str.starts_with("/proc")
            || path_str.starts_with("C:\\Windows")
            || path_str.starts_with("C:\\Program Files")
        {
            return Err(format!(
                "Access to system directory '{}' is not allowed",
                path
            ));
        }
    }

    Ok(path_obj.to_path_buf())
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

    #[test]
    fn test_validate_output_path_normal() {
        assert!(validate_output_path("./bin").is_ok());
        assert!(validate_output_path("output").is_ok());
        assert!(validate_output_path("./output/dir").is_ok());
    }

    #[test]
    fn test_validate_output_path_rejects_traversal() {
        assert!(validate_output_path("../etc").is_err());
        assert!(validate_output_path("../../etc/passwd").is_err());
        assert!(validate_output_path("foo/../bar").is_err());
    }

    #[test]
    fn test_validate_output_path_rejects_system_dirs() {
        assert!(validate_output_path("/etc/config").is_err());
        assert!(validate_output_path("/sys/kernel").is_err());
        assert!(validate_output_path("/proc/self").is_err());
    }

    #[test]
    fn test_validate_output_path_windows() {
        if cfg!(windows) {
            assert!(validate_output_path("C:\\Windows\\System32").is_err());
            assert!(validate_output_path("C:\\Program Files\\test").is_err());
        }
    }
}

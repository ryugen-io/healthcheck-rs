//! Secure path validation preventing path traversal and TOCTOU attacks.
//!
//! Blocks writes to system dirs (/etc, /bin, /sys, /proc, C:\Windows, etc.),
//! resolves symlinks via canonicalization, and handles extensionless files.

use std::env;
use std::path::{Path, PathBuf};

/// Validate and safely resolve output path to prevent path traversal attacks
/// Uses canonicalization to prevent TOCTOU vulnerabilities
///
/// This function is public for testing purposes
pub(crate) fn validate_output_path(path: &str) -> Result<PathBuf, String> {
    let path_obj = Path::new(path);

    // Determine if this is a file or directory path
    // If path exists, check directly; otherwise, infer from trailing slash or context
    let (dir_to_check, is_file) = if path_obj.exists() {
        // Path exists - check if it's a file or directory
        if path_obj.is_file() {
            (path_obj.parent().unwrap_or_else(|| Path::new(".")), true)
        } else {
            (path_obj, false)
        }
    } else {
        // Path doesn't exist - infer from pattern
        // Directories typically end with / or \
        // Files have extensions OR are extensionless but don't end with path separator
        let ends_with_separator = path.ends_with('/') || path.ends_with('\\');

        if ends_with_separator {
            // Explicitly a directory
            (path_obj, false)
        } else {
            // Assume it's a file (handles both "config.txt" and "Makefile")
            (path_obj.parent().unwrap_or_else(|| Path::new(".")), true)
        }
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

        if let Some(dir_name) = dir_to_check.file_name() {
            parent_canonical.join(dir_name)
        } else {
            // Simple path with no parent (e.g., "bin" or "output")
            let dir_str = dir_to_check.to_string_lossy();
            parent_canonical.join(dir_str.as_ref())
        }
    };

    // Check against system directories after canonicalization
    let canonical_str = canonical.to_string_lossy();

    #[cfg(unix)]
    {
        // Block access to critical system directories
        let protected_dirs = [
            "/etc",      // System configuration
            "/sys",      // Kernel/system information
            "/proc",     // Process information
            "/dev",      // Device files
            "/boot",     // Boot files
            "/bin",      // Essential binaries
            "/sbin",     // System binaries
            "/lib",      // System libraries
            "/lib64",    // 64-bit system libraries
            "/usr/bin",  // User binaries
            "/usr/sbin", // User system binaries
            "/usr/lib",  // User libraries
            "/run",      // Runtime data (canonical target of /var/run)
            "/var/run",  // Runtime data (often symlinked to /run)
            "/var/lock", // Lock files
            "/root",     // Root home directory
        ];

        for dir in &protected_dirs {
            // Check for exact match or directory followed by /
            // This prevents false positives like /etc-backup or /binary
            if canonical_str == *dir || canonical_str.starts_with(&format!("{}/", dir)) {
                return Err(format!(
                    "Access to system directory '{}' is not allowed",
                    canonical_str
                ));
            }
        }
    }

    #[cfg(windows)]
    {
        let lower = canonical_str.to_lowercase();
        let protected_dirs = [
            "c:\\windows",
            "c:\\program files",
            "c:\\program files (x86)",
            "c:\\programdata\\microsoft",
            "c:\\system volume information",
        ];

        for dir in &protected_dirs {
            // Check for exact match or directory followed by \
            // This prevents false positives like c:\windows-backup
            if lower == *dir || lower.starts_with(&format!("{}\\", dir)) {
                return Err(format!(
                    "Access to system directory '{}' is not allowed",
                    canonical_str
                ));
            }
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

#[cfg(test)]
mod tests;

#[cfg(test)]
mod edge_case_tests;

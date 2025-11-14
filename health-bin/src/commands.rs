use std::env;
use std::fs;
use std::path::Path;

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

    // Determine output directory
    let out_dir = output_dir.as_deref().unwrap_or("./bin");
    fs::create_dir_all(out_dir)
        .map_err(|e| format!("Failed to create output directory '{}': {}", out_dir, e))?;

    // Generate platform-specific binary name
    let binary_name = format!("healthcheck-{}-{}{}", os, arch, ext);
    let output_path = Path::new(out_dir).join(&binary_name);

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

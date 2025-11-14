use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn print_version() {
    println!("healthcheck v{}", VERSION);
}

pub fn print_help() {
    println!("healthcheck v{} - Lightweight health check tool", VERSION);
    println!();
    println!("USAGE:");
    println!("    healthcheck [OPTIONS] [CONFIG_FILE]");
    println!("    healthcheck <COMMAND>");
    println!();
    println!("ARGS:");
    println!("    <CONFIG_FILE>    Path to config file [default: healthcheck.config]");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help       Print help information");
    println!("    -v, --version    Print version information");
    println!();
    println!("COMMANDS:");
    println!("    generate-bin     Generate standalone binary for deployment");
    println!("    serve            Start HTTP API server (coming soon)");
    println!("    watch            Watch mode with continuous monitoring (coming soon)");
    println!();
    println!("EXAMPLES:");
    println!("    # Run health checks from config");
    println!("    healthcheck myconfig.conf");
    println!();
    println!("    # Generate binary for container deployment");
    println!("    healthcheck generate-bin");
    println!("    healthcheck generate-bin --output ./bin");
    println!();
    println!("CONFIG FORMAT:");
    println!("    tcp:host=localhost,port=8080,timeout_ms=1000");
    println!("    http:url=http://localhost:8080/health,timeout_ms=5000");
    println!("    database:conn_str=postgresql://user:pass@localhost/db");
    println!("    process:name=myapp");
}

pub enum CliAction {
    Help,
    Version,
    GenerateBin { output_dir: Option<String> },
    Serve,
    Watch,
    RunChecks { config_path: String },
}

pub fn parse_args() -> CliAction {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => return CliAction::Help,
            "-v" | "--version" => return CliAction::Version,
            "generate-bin" => {
                let output_dir = if args.len() > 2 && args[2] == "--output" {
                    args.get(3).cloned()
                } else {
                    None
                };
                return CliAction::GenerateBin { output_dir };
            }
            "serve" => return CliAction::Serve,
            "watch" => return CliAction::Watch,
            path => {
                return CliAction::RunChecks {
                    config_path: path.to_string(),
                };
            }
        }
    }

    CliAction::RunChecks {
        config_path: "healthcheck.config".to_string(),
    }
}

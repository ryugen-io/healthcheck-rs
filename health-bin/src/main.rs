mod cli;
mod commands;
mod runner;
mod status;

use cli::{CliAction, parse_args, print_help, print_version};

fn main() {
    match parse_args() {
        CliAction::Help => print_help(),
        CliAction::Version => print_version(),
        CliAction::GenerateBin { output_dir } => match commands::generate_bin(output_dir) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("❌ Error: {}", e);
                std::process::exit(1);
            }
        },
        CliAction::Serve => {
            eprintln!("❌ 'serve' command not yet implemented");
            eprintln!("   Coming soon: HTTP API server mode");
            std::process::exit(1);
        }
        CliAction::Watch => {
            eprintln!("❌ 'watch' command not yet implemented");
            eprintln!("   Coming soon: Continuous monitoring mode");
            std::process::exit(1);
        }
        CliAction::RunChecks { config_path } => {
            runner::run_health_checks(&config_path);
        }
    }
}

//! Installer Analyzer - Main Entry Point

use installer_analyzer::cli::{Cli, Commands};
use installer_analyzer::cli::commands;
use installer_analyzer::utils;
use clap::Parser;
use std::process;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize logging
    if let Err(e) = utils::init_logging(cli.verbose) {
        eprintln!("Failed to initialize logging: {}", e);
        process::exit(1);
    }

    // Check for admin privileges for sandbox operations
    if matches!(cli.command, Commands::Sandbox { .. }) && !utils::is_admin() {
        eprintln!("Warning: Sandbox analysis requires administrator privileges for full functionality.");
        eprintln!("Some features may not work correctly without elevated permissions.");
    }

    // Execute command
    let result = match cli.command {
        Commands::Analyze { input, output, format } => {
            commands::handle_analyze(&input, output.as_deref(), &format).await
        }
        Commands::Sandbox { input, output, format, timeout, network } => {
            commands::handle_sandbox(&input, output.as_deref(), &format, timeout, network).await
        }
        Commands::Batch { input_dir, output_dir, format, sandbox } => {
            commands::handle_batch(&input_dir, &output_dir, &format, sandbox).await
        }
        Commands::Info => {
            commands::handle_info().await
        }
    };

    // Handle result
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

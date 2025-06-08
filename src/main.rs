//! Installer Analyzer - Main Entry Point

use clap::Parser;
use installer_analyzer::cli::commands;
use installer_analyzer::cli::{Cli, Commands};
use installer_analyzer::cli::output::{CliOutput, init_console};
use installer_analyzer::utils;
use std::process;

#[tokio::main]
async fn main() {
    // Initialize console for better Windows compatibility
    init_console();

    let cli = Cli::parse();

    // Show startup banner
    CliOutput::startup_banner(env!("CARGO_PKG_VERSION"));

    // Initialize logging
    if let Err(e) = utils::init_logging(cli.verbose) {
        eprintln!("Failed to initialize logging: {}", e);
        process::exit(1);
    }

    // Check for admin privileges for sandbox operations
    if matches!(cli.command, Commands::Sandbox { .. }) && !utils::is_admin() {
        eprintln!(
            "Warning: Sandbox analysis requires administrator privileges for full functionality."
        );
        eprintln!("Some features may not work correctly without elevated permissions.");
    }

    // Execute command
    let result = match cli.command {
        Commands::Analyze {
            input,
            output,
            format,
            open,
        } => commands::handle_analyze(&input, output.as_deref(), format.as_deref(), open).await,
        Commands::Sandbox {
            input,
            output,
            format,
            timeout,
            network,
            open,
        } => {
            commands::handle_sandbox(&input, output.as_deref(), format.as_deref(), timeout, network, open)
                .await
        }
        Commands::Batch {
            input_dir,
            output_dir,
            format,
            sandbox,
        } => commands::handle_batch(&input_dir, &output_dir, format.as_deref(), sandbox).await,
        Commands::Info => commands::handle_info().await,
    };

    // Handle result
    if let Err(e) = result {
        CliOutput::error(&format!("Error: {}", e));
        process::exit(1);
    }
}

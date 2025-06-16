//! Command line interface

use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod commands;
pub mod output;

/// Installer Analyzer CLI
#[derive(Parser)]
#[command(name = "installer-analyzer")]
#[command(about = "A comprehensive tool for analyzing software installation packages")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Perform static analysis of an installer
    Analyze {
        /// Path to the installer file
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output format (json, html, markdown). Auto-detected from file extension if not specified.
        #[arg(short, long)]
        format: Option<String>,

        /// Automatically open HTML report in browser
        #[arg(long)]
        open: bool,
    },

    /// Run installer in sandbox for dynamic analysis
    Sandbox {
        /// Path to the installer file
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output format (json, html, markdown). Auto-detected from file extension if not specified.
        #[arg(short, long)]
        format: Option<String>,

        /// Maximum execution time in seconds
        #[arg(short, long, default_value = "300")]
        timeout: u64,

        /// Enable network monitoring
        #[arg(short, long)]
        network: bool,

        /// Automatically open HTML report in browser
        #[arg(long)]
        open: bool,
    },

    /// Batch process multiple installers
    Batch {
        /// Directory containing installer files
        #[arg(short, long)]
        input_dir: PathBuf,

        /// Output directory for reports
        #[arg(short, long)]
        output_dir: PathBuf,

        /// Output format (json, html, markdown). Defaults to json for batch processing.
        #[arg(short, long)]
        format: Option<String>,

        /// Use sandbox analysis
        #[arg(short, long)]
        sandbox: bool,
    },

    /// Show information about supported formats
    Info,

    /// Check for and install updates
    Update {
        /// Only check for updates without installing
        #[arg(long)]
        check_only: bool,

        /// Force update even if current version is newer
        #[arg(long)]
        force: bool,

        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },
}

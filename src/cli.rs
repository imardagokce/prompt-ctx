use std::path::PathBuf;

use clap::Parser;

/// Collect and count the text context in a directory.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Target directory to scan.
    #[arg(value_name = "PATH", default_value = ".")]
    pub path: Option<PathBuf>,

    /// Maximum token budget.
    #[arg(short = 'm', long = "max-tokens", value_name = "INT")]
    pub max_tokens: Option<usize>,

    /// Keep only code signatures and structures.
    #[arg(short = 's', long)]
    pub signatures_only: bool,

    /// Include only files reported as changed by git status.
    #[arg(short = 'd', long)]
    pub diff_only: bool,

    /// Write output to this file.
    #[arg(short = 'o', long, value_name = "PATH")]
    pub output: Option<PathBuf>,
}

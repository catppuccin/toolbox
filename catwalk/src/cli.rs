#![deny(clippy::perf, clippy::nursery, clippy::pedantic)]
use catppuccin_catwalk::Layout;
use clap::{Command, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Generator, Shell};
use std::path::PathBuf;

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    #[command(about = "Generates a completion file for the given shell")]
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Extension {
    Webp,
    Png,
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Latte image
    #[arg(default_value = "latte.webp")]
    pub latte: Option<PathBuf>,
    /// Frappé image
    #[arg(default_value = "frappe.webp")]
    pub frappe: Option<PathBuf>,
    /// Macchiato image
    #[arg(default_value = "macchiato.webp")]
    pub macchiato: Option<PathBuf>,
    /// Mocha image
    #[arg(default_value = "mocha.webp")]
    pub mocha: Option<PathBuf>,
    /// Output file
    #[arg(short, long, default_value = "preview.webp")]
    pub output: PathBuf,
    /// Layout
    #[arg(short, long, value_enum, default_value_t=Layout::Composite)]
    pub layout: Layout,
    /// Sets the radius (percentage)
    #[arg(short, long)]
    pub radius: Option<u32>,
    /// Gap (grid layout)
    #[arg(short, long)]
    pub gap: Option<u32>,
    /// Change to <DIRECTORY> before processing files
    #[arg(short = 'C', long, default_value = ".")]
    pub directory: Option<PathBuf>,
    /// Extension to use when auto-detecting formats
    #[arg(long = "ext", value_enum, default_value_t = Extension::Webp)]
    pub extension: Extension,

    // Shell completion
    #[command(subcommand)]
    pub command: Option<Commands>,
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

pub fn get_cli_arguments() -> Cli {
    Cli::parse()
}

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
    /// Path to Latte image.
    #[arg(default_value = "./latte.webp")]
    pub latte: Option<PathBuf>,
    /// Path to Frappé image.
    #[arg(default_value = "./frappe.webp")]
    pub frappe: Option<PathBuf>,
    /// Path to Macchiato image.
    #[arg(default_value = "./macchiato.webp")]
    pub macchiato: Option<PathBuf>,
    /// Path to Mocha image.
    #[arg(default_value = "./mocha.webp")]
    pub mocha: Option<PathBuf>,
    /// Path to output file.
    #[arg(short, long, default_value = "./preview.webp")]
    pub output: PathBuf,
    /// Layout to use.
    #[arg(short, long, value_enum, default_value_t=Layout::Composite)]
    pub layout: Layout,
    /// Radius of rounded corners (percentage).
    #[arg(short, long)]
    pub radius: Option<u32>,
    /// Size of gaps between pictures for the `grid` layout.
    #[arg(short, long)]
    pub gap: Option<u32>,
    /// Change to <DIRECTORY> before processing files.
    #[arg(short = 'C', long, default_value = ".")]
    pub directory: Option<PathBuf>,
    /// File extension to use for input files, if they're not explicitly named.
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

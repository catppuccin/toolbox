use clap::{Args, Command, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Generator, Shell};
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Layout {
    Composite,
    Stacked,
    Grid,
}

#[derive(Args, Clone, Debug)]
#[command(args_conflicts_with_subcommands(true))]
pub struct ImageArgs {
    pub latte: PathBuf,
    pub frappe: PathBuf,
    pub macchiato: PathBuf,
    pub mocha: PathBuf,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Commands {
    #[command(about = "Generates a completion file for the given shell")]
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Cli {
    /// Image inputs
    #[command(flatten)]
    pub images: Option<ImageArgs>,
    /// Output file
    #[arg(short, long, default_value = "result.webp")]
    pub output: PathBuf,
    /// Layout
    #[arg(short, long, value_enum, default_value_t=Layout::Composite)]
    pub layout: Layout,
    /// Gap (grid layout)
    #[arg(short, long, default_value_t = 50)]
    pub gap: u32,
    /// Sets the radius (percentage)
    #[arg(short, long, default_value_t = 3)]
    pub radius: u32,
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

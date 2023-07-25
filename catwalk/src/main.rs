use catwalk::Magic;
use clap::{Args, Command, CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Generator, Shell};
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use image::{open, RgbaImage};
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Layout {
    Composite,
    Stacked,
    Grid,
}

#[derive(Args, Clone, Debug)]
#[command(args_conflicts_with_subcommands(true))]
struct ImageArgs {
    latte: PathBuf,
    frappe: PathBuf,
    macchiato: PathBuf,
    mocha: PathBuf,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    #[command(about = "Generates a completion file for the given shell")]
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    /// Image inputs
    #[command(flatten)]
    images: Option<ImageArgs>,
    /// Output file
    #[arg(short, long, default_value = "result.webp")]
    output: PathBuf,
    /// Layout
    #[arg(short, long, value_enum, default_value_t=Layout::Composite)]
    layout: Layout,
    /// Gap (grid layout)
    #[arg(short, long, default_value_t = 150)]
    gap: u32,
    /// Sets the radius.
    #[arg(short, long, default_value_t = 75)]
    radius: u32,
    // Shell completion
    #[command(subcommand)]
    command: Option<Commands>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

fn open_rgba_image(path: &Path) -> Result<RgbaImage> {
    Ok(open(path)?.to_rgba8())
}

fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .panic_section("Consider reporting this issue to https://github.com/catppuccin/toolbox")
        .display_env_section(false)
        .install()?;

    let args = Cli::parse();

    if let Some(generator) = args.command {
        match generator {
            Commands::Completion { shell } => {
                let mut cmd = Cli::command();
                eprintln!("Generating completion file for {generator:?}...");
                print_completions(shell, &mut cmd);
                return Ok(());
            }
        }
    }

    let images = args.images.ok_or_else(|| eyre!("No images provided"))?;

    let magic = Magic::new(
        [
            open_rgba_image(&images.latte).context("Failed to open Latte image")?,
            open_rgba_image(&images.frappe).context("Failed to open Frappé image")?,
            open_rgba_image(&images.macchiato).context("Failed to open Macchiato image")?,
            open_rgba_image(&images.mocha).context("Failed to open Mocha image")?,
        ],
        args.radius,
    )?;

    (match args.layout {
        Layout::Composite => magic.gen_composite(),
        Layout::Stacked => magic.gen_stacked(),
        Layout::Grid => magic.gen_grid(args.gap),
    })
    .save(args.output)?;

    Ok(())
}

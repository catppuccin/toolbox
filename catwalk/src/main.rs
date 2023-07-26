mod cli;

use catwalk::Magic;
use clap::CommandFactory;

use cli::{get_cli_arguments, print_completions, Cli, Commands, Layout};
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use image::{open, RgbaImage};
use std::path::Path;

fn open_rgba_image(path: &Path) -> Result<RgbaImage> {
    Ok(open(path)?.to_rgba8())
}

fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .panic_section("Consider reporting this issue to https://github.com/catppuccin/toolbox")
        .display_env_section(false)
        .install()?;

    let args = get_cli_arguments();

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

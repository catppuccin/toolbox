mod cli;

use catwalk::Magic;
use clap::CommandFactory;
use std::io::Cursor;

use cli::{get_cli_arguments, print_completions, Cli, Commands, Layout};
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use ril::{encodings::webp, prelude::*};
use std::path::Path;

fn open_rgba_image(path: &Path) -> Result<Image<Rgba>> {
    Image::<Rgba>::open(path).map_or(Err(eyre!("Failed to open image")), Ok)
}

fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .panic_section(format!(
            "Consider reporting this issue to {}",
            env!("CARGO_PKG_REPOSITORY")
        ))
        .display_env_section(false)
        .install()?;

    let args = get_cli_arguments();

    if let Some(generator) = args.command {
        return match generator {
            Commands::Completion { shell } => {
                let mut cmd = Cli::command();
                eprintln!("Generating completion file for {generator:?}...");
                print_completions(shell, &mut cmd);
                Ok(())
            }
        };
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

    if args.output.extension().unwrap_or_default() != "webp" {
        return Err(eyre!("Output file must be a .webp file"));
    }

    let buffer = match args.layout {
        Layout::Composite => magic.gen_composite(),
        Layout::Stacked => magic.gen_stacked(),
        Layout::Grid => magic.gen_grid(args.gap),
    };

    let mut writebuf = Cursor::new(Vec::new());
    let encoder_options = webp::WebPEncoderOptions::new().with_lossless(true);
    let encoder_metadata = EncoderMetadata::<Rgba>::from(&buffer).with_config(encoder_options);
    let mut encoder = webp::WebPStaticEncoder::new(&mut writebuf, encoder_metadata)?;
    encoder.add_frame(&buffer)?;

    std::fs::write(args.output, writebuf.get_ref())
        .map_err(|e| eyre!("Failed to write image: {}", e))
}

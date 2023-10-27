#![cfg(not(target_family = "wasm"))]
mod cli;

use catppuccin_catwalk::Catwalk;
use clap::CommandFactory;
use cli::{get_cli_arguments, print_completions, Cli, Commands};
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use ril::prelude::*;
use std::{io::Cursor, path::Path};

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

    let catwalk = Catwalk::new([
        open_rgba_image(&images.latte).context("Failed to open Latte image")?,
        open_rgba_image(&images.frappe).context("Failed to open Frappé image")?,
        open_rgba_image(&images.macchiato).context("Failed to open Macchiato image")?,
        open_rgba_image(&images.mocha).context("Failed to open Mocha image")?,
    ])?
    .gap(args.gap)
    .layout(args.layout)
    .radius(args.radius)?
    .build()?;

    let mut writebuf = Cursor::new(Vec::new());
    match args.output.extension() {
        None => return Err(eyre!("Output file type could not be determined")),
        Some(os_str) => match os_str.to_str() {
            Some("png") => {
                use ril::encodings::png::PngEncoder;

                PngEncoder::encode_static(&catwalk, &mut writebuf)?;
            }
            Some("webp") => {
                use ril::encodings::webp::{WebPEncoderOptions, WebPStaticEncoder};

                let opt = WebPEncoderOptions::new().with_lossless(true);
                let meta = EncoderMetadata::<Rgba>::from(&catwalk).with_config(opt);
                let mut encoder = WebPStaticEncoder::new(&mut writebuf, meta)?;
                encoder.add_frame(&catwalk)?;
            }
            _ => return Err(eyre!("Output file type not supported")),
        },
    }

    std::fs::write(args.output, writebuf.get_ref())
        .map_err(|e| eyre!("Failed to write image: {}", e))
}

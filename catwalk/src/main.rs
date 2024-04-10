#![deny(clippy::perf, clippy::nursery, clippy::pedantic)]
mod cli;

use catppuccin_catwalk::Catwalk;
use clap::CommandFactory;
use cli::{get_cli_arguments, print_completions, Cli, Commands, Extension};
use color_eyre::{eyre::eyre, Result};
use ril::prelude::*;
use std::io::Cursor;

macro_rules! open_image {
    ($path:expr, $args:expr) => {{
        let mut rel_path = $args.directory.clone().unwrap_or_default();
        let path = $path.unwrap_or_default();
        rel_path.push(path.clone());
        // set the `--ext` file extension unless the filenames are explicitly given
        if path == std::path::PathBuf::default() {
            match $args.extension {
                Extension::Webp => {
                    rel_path.set_extension("webp");
                }
                Extension::Png => {
                    rel_path.set_extension("png");
                }
            }
        }
        Image::<Rgba>::open(&rel_path)
            .map_or(Err(eyre!("Failed to open `{}`", &rel_path.display())), Ok)?
    }};
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

    let catwalk = Catwalk::new([
        open_image!(args.latte, args),
        open_image!(args.frappe, args),
        open_image!(args.macchiato, args),
        open_image!(args.mocha, args),
    ])?
    .gap(args.gap)
    .layout(Some(args.layout))
    .radius(args.radius)?
    .build()?;

    let mut writebuf = Cursor::new(Vec::new());
    match args.output.extension() {
        None => return Err(eyre!("Output file type could not be determined")),
        Some(os_str) => match os_str.to_str() {
            Some("png") => catwalk.encode(ImageFormat::Png, &mut writebuf)?,
            Some("webp") => catwalk.encode(ImageFormat::WebP, &mut writebuf)?,
            _ => return Err(eyre!("Output file type not supported")),
        },
    };

    let output = if args.directory.is_some() {
        let mut path = args.directory.clone().unwrap_or_default();
        if args.output.is_absolute() {
            args.output
        } else {
            path.push(args.output);
            path
        }
    } else {
        args.output
    };

    catwalk
        .save_inferred(output)
        .map_err(|e| eyre!("Failed to write image: {}", e))
}

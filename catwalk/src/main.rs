use catwalk::Magic;
use clap::{Args, Command, CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Generator, Shell};
use image::open;
use rayon::prelude::*;
use std::path::PathBuf;

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
    #[arg(short, long)]
    output: Option<PathBuf>,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    if let Some(generator) = args.command {
        match generator {
            Commands::Completion { shell } => {
                let mut cmd = Cli::command();
                eprintln!("Generating completion file for {generator:?}...");
                print_completions(shell, &mut cmd);
                std::process::exit(0);
            }
        }
    }

    let magic = Magic::new(
        args.images
            .map_or_else(
                || {
                    eprintln!("No images provided");
                    std::process::exit(1);
                },
                |x| vec![x.latte, x.frappe, x.macchiato, x.mocha],
            )
            .par_iter()
            .map(open)
            .map(|x| {
                x.unwrap_or_else(|e| {
                    eprintln!("Failed to open image: {}", e);
                    std::process::exit(1)
                })
                .to_rgba8()
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("Failed to convert images to array"),
        args.radius,
    )?;

    (match args.layout {
        Layout::Composite => magic.gen_composite(),
        Layout::Stacked => magic.gen_stacked(),
        Layout::Grid => magic.gen_grid(args.gap),
    })
    .save(args.output.unwrap_or_else(|| PathBuf::from("result.webp")))?;

    Ok(())
}

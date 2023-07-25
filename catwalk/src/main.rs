use catwalk::Magic;
use clap::{Command, CommandFactory};
pub use clap::{Parser, ValueEnum};
use clap_complete::Shell;
use clap_complete::{generate, Generator};
use image::open;
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Layout {
    Composite,
    Stacked,
    Grid,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Args {
    /// Latte screenshot
    pub latte: PathBuf,
    /// Frappe screenshot
    pub frappe: PathBuf,
    /// Macchiato screenshot
    pub macchiato: PathBuf,
    /// Mocha screenshot
    pub mocha: PathBuf,
    /// Layout
    #[arg(short, long, value_enum, default_value_t=Layout::Composite)]
    pub layout: Layout,
    /// Gap (grid layout)
    #[arg(short, long, default_value_t = 150)]
    pub gap: u32,
    /// Sets the radius.
    #[arg(short, long, default_value_t = 75)]
    pub radius: u32,
    // Shell completion
    #[arg(long, value_enum)]
    completion: Option<Shell>,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

fn main() {
    let args = Args::parse();

    if let Some(generator) = args.completion {
        let mut cmd = Args::command();
        eprintln!("Generating completion file for {generator:?}...");
        print_completions(generator, &mut cmd);
        return;
    }

    let magic = Magic::new(
        [args.latte, args.frappe, args.macchiato, args.mocha]
            .map(open)
            .map(|x| {
                x.unwrap_or_else(|_| {
                    eprintln!("Failed to open image");
                    std::process::exit(1)
                })
                .to_rgba8()
            }),
    );

    (match args.layout {
        Layout::Composite => magic.gen_composite(args.radius),
        Layout::Stacked => magic.gen_stacked(args.radius),
        Layout::Grid => magic.gen_grid(args.radius, args.gap),
    })
    .save("result.webp")
    .unwrap();
}

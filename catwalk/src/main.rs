use catwalk::{Args, Magic, Parser, MagicTricks};

fn main() {
    let mut args = Args::parse();
    let magic = Magic::from(args.clone());
    args.background = args.background.replace("#", "");
    if args.background.len() == 6 {
        args.background += "ff";
    }
    let result = magic.gen_slants().margin(args.margin, args.background);
    result.save("result.png").unwrap();
}

use catwalk::{Args, Magic, Parser, MagicBuf, MagicTricks, RoundMask};

fn main() {
    let mut args = Args::parse();
    let magic = Magic::from(args.clone());
    args.background = args.background.replace("#", "");
    if args.background.len() == 6 {
        args.background += "ff";
    }
    if args.outer < 0 {
        args.outer = args.radius as i32;
    }
    let mut result: MagicBuf;
    if args.layout == "composite".to_string() {
        result = magic.gen_composite(args.radius);
    } else if args.layout == "stacked".to_string() {
        result = magic.gen_stacked(args.radius);
    } else if args.layout == "grid".to_string() {
        result = magic.gen_grid(args.radius, args.gap);
    } else {
        panic!("Invalid layout: {}", args.layout);
    }
    let round_mask = RoundMask { radius: args.outer as u32 };
    result = round_mask.mask(&result.margin(args.margin, args.background));
    result.save("result.webp").unwrap();
}

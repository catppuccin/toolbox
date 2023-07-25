use catwalk::{Args, Magic, MagicBuf, Parser};

fn main() {
    let args = Args::parse();
    let magic = Magic::from(args.clone());
    let result: MagicBuf;
    if args.layout == *"composite" {
        result = magic.gen_composite(args.radius);
    } else if args.layout == *"stacked" {
        result = magic.gen_stacked(args.radius);
    } else if args.layout == *"grid" {
        result = magic.gen_grid(args.radius, args.gap);
    } else {
        panic!("Invalid layout: {}", args.layout);
    }
    result.save("result.webp").unwrap();
}

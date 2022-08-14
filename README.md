# 🖼️ Puccinier

If you wrote a v1 or v2 Catppuccin theme, you may use Puccinier to automatically create the the rest of the v2 themes. Puccinier supports the conversion of properly formatted hex, rgb, and hsl colors (case insensitive).

## Help info

```bash
puccinier 0.1.2
ap-1 <anishp0828@gmail.com>
Generate the rest of the Catppuccin palettes off of a template file written in one of them

USAGE:
    puccinier --source <FILE> --output <TYPES>...

OPTIONS:
    -h, --help                 Print help information
    -o, --output <TYPES>...    Set the themes (space-separated) to generate from the source file
                               [possible values: latte, frappe, macchiato, mocha]
    -s, --source <FILE>        Set the source file to convert
    -V, --version              Print version information
```

## Usage

You can find binaries built for `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, and `x86_64-pc-windows-msvc` in the releases tab. Alternatively, you can use [cargo](https://github.com/rust-lang/cargo/):

```bash
cargo install puccinier
```

To build from source, you must use cargo:

```bash
git clone https://github.com/ap-1/puccinier && cd puccinier
cargo install --path .
```

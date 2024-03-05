<h3 align="center">
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
  Catppuccin Catwalk
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
  <a href="https://github.com/catppuccin/toolbox/stargazers"><img src="https://img.shields.io/github/stars/catppuccin/toolbox?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/toolbox/issues?label=catwalk"><img src="https://img.shields.io/github/issues/catppuccin/toolbox?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/toolbox/contributors"><img src="https://img.shields.io/github/contributors/catppuccin/toolbox?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

&nbsp;

A sweet program that takes in four showcase images and displays them all at once.

## Installation

You can install Catwalk using one of the methods below:

| Installation Method                   | Instructions                                                                                                                  |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| crates.io                             | `cargo install catppuccin-catwalk`                                                                                            |
| Source                                | `cargo install --git https://github.com/catppuccin/toolbox catppuccin-catwalk`                                                |
| Homebrew                              | `brew install catppuccin/tap/catwalk`                                                                                         |
| Nix                                   | `nix profile install github:catppuccin/toolbox#catwalk`<br/>`nix run github:catppuccin/toolbox#catwalk -- [OPTIONS] <images>` |
| Binaries<br/>(Windows, MacOS & Linux) | Available from the [latest GitHub release](https://github.com/catppuccin/toolbox/releases?q=catwalk).                         |

## Usage

```console
$ catwalk --help
A sweet program that takes in four showcase images and displays them all at once.

Usage: catwalk [OPTIONS] [LATTE] [FRAPPE] [MACCHIATO] [MOCHA] [COMMAND]

Commands:
  completion  Generates a completion file for the given shell
  help        Print this message or the help of the given subcommand(s)

Arguments:
  [LATTE]      Path to Latte image [default: ./latte.webp]
  [FRAPPE]     Path to Frappé image [default: ./frappe.webp]
  [MACCHIATO]  Path to Macchiato image [default: ./macchiato.webp]
  [MOCHA]      Path to Mocha image [default: ./mocha.webp]

Options:
  -o, --output <OUTPUT>        Path to output file [default: ./preview.webp]
  -l, --layout <LAYOUT>        Layout to use [default: composite] [possible values: composite, stacked, grid, row]
  -r, --radius <RADIUS>        Radius of rounded corners (percentage)
  -g, --gap <GAP>              Size of gaps between pictures for the `grid` layout
  -C, --directory <DIRECTORY>  Change to <DIRECTORY> before processing files [default: .]
      --ext <EXTENSION>        File extension to use for input files, if they're not explicitly named [default: webp] [possible values: webp, png]
  -h, --help                   Print help
  -V, --version                Print version
```

### Examples

```console
$ catwalk latte.webp frappe.webp macchiato.webp mocha.webp --output catwalk.webp
```

```console
$ catwalk latte.png frappe.png macchiato.png mocha.png --directory ./assets/
```

&nbsp;

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2021-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
<p align="center"><a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=302d41&colorB=b7bdf8"/></a></p>

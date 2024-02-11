<h3 align="center">
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
  Catppuccin Puccinier
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
  <a href="https://github.com/catppuccin/toolbox/stargazers"><img src="https://img.shields.io/github/stars/catppuccin/toolbox?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/toolbox/issues"><img src="https://img.shields.io/github/issues/catppuccin/toolbox?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/toolbox/contributors"><img src="https://img.shields.io/github/contributors/catppuccin/toolbox?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

&nbsp;

If you wrote a v1 or v2 Catppuccin theme, you may use Puccinier to automatically create the rest of the v2 themes. Puccinier supports the conversion of properly formatted hex, rgb, and hsl colors (case insensitive).

## Installation

You can install Puccinier using one of the methods below:

| Installation Method                   | Instructions                                                                                                                      |
| ------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| crates.io                             | `cargo install puccinier`                                                                                                         |
| Source                                | `cargo install --git https://github.com/catppuccin/toolbox puccinier`                                                             |
| Nix                                   | `nix profile install github:catppuccin/toolbox#puccinier`<br/>`nix run github:catppuccin/toolbox#puccinier -- [OPTIONS] <source>` |
| Binaries<br/>(Windows, MacOS & Linux) | Available from the [latest GitHub release](https://github.com/catppuccin/toolbox/releases?q=puccinier).                           |

## Usage

```console
$ puccinier
puccinier
  Generate the other Catppuccin flavours off a template file written in one of them

ARGS:
    <source>
      The source file to convert

OPTIONS:
    -o, --output <type>
      Set the themes to generate from the source file (one of 'latte', 'frappe', 'macchiato', or 'mocha')

    -s, --silent
      Disable showing which parts of the file were replaced

    -h, --help
      Prints help information.
```

&nbsp;

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2021-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
<p align="center"><a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=302d41&colorB=b7bdf8"/></a></p>

<h3 align="center">
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
  Catppuccin Catwalk
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
  <a href="https://github.com/catppuccin/toolbox/stargazers"><img src="https://img.shields.io/github/stars/catppuccin/toolbox?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/toolbox/issues"><img src="https://img.shields.io/github/issues/catppuccin/toolbox?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/toolbox/contributors"><img src="https://img.shields.io/github/contributors/catppuccin/toolbox?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

&nbsp;

A sweet program that takes in four showcase images and displays them all at once.

Installation with Cargo, Nix, or from source:

```console
$ cargo install catppuccin-catwalk
$ catwalk <images> <flags>
# to install from source:
$ cargo install --git https://github.com/catppuccin/toolbox catwalk
# there's also a Nix flake:
$ nix run github:catppuccin/toolbox#catwalk -- <images> <flags>
```

Usage:

Parameter     | Description
--------------| ------------------------------------------------------------------------------------------------------- 
`[latte]`     | Path to Latte image (default: `./latte.webp`)
`[frappe]`    | Path to Frappé image (default: `./frappe.webp`)
`[macchiato]` | Path to Macchiato image (default: `./macchiato.webp`)
`[mocha]`     | Path to Mocha image (default: `./mocha.webp`)
`--output`    | Path to output file (default: `./preview.webp`)
`--layout`    | Style of the showcase image. Available options are `composite` (default), `grid`, `row`, and `stacked`.
`--radius`    | Radius of rounded corners (percentage).
`--directory` | Change to `<DIRECTORY>` before processing files. (e.g. `catwalk -C ./assets/`)
`--ext`       | File extension to use for input files, if they're not explicitly named. `webp` (default) or `png`.
`--gap`       | Size of gaps between pictures for the `grid` layout.
`--help`      | A summary of the available parameters.

&nbsp;

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2021-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
<p align="center"><a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=302d41&colorB=b7bdf8"/></a></p>

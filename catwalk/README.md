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

Installation with Cargo or Nix:

```bash
$ cargo install catppuccin-catwalk
$ catwalk <images> <flags>
# There's also a Nix flake option
$ nix run github:catppuccin/toolbox#catwalk -- <images> <flags>
```

To install from source, you can use cargo:

| Parameter      | Description                                                                                      |
| -------------- | -------------------------------------------------------------------------------------------------|
| `images[4]`    | 4 images to merge into one. **REQUIRED**. *All other parameters are optional.*                   |
| `--layout`     | Style of the showcase image. Available options are `composite` (default), `grid`, and `stacked`. |
| `--gap`        | Gap size for the `grid` layout.                                                                  |
| `--radius`     | Radius of rounded corners.                                                                       |
| `--output`     | Output file (defaults to `./result.webp`)                                                        |
| `--help`       | A summary of the available parameters.                                                           |

&nbsp;

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2021-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
<p align="center"><a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=302d41&colorB=b7bdf8"/></a></p>

<p align="center">
  <h2 align="center">😸🧰 Toolbox</h2>
</p>

<p align="center">
	Catppuccin's developement tools
</p>

<p align="center">
	<a href="https://github.com/catppuccin/catppuccin/stargazers">
		<img alt="Stars" src="https://img.shields.io/github/stars/catppuccin/catppuccin?style=for-the-badge&logo=starship&color=C9CBFF&logoColor=D9E0EE&labelColor=302D41"></a>
	<a href="https://github.com/catppuccin/catppuccin/issues">
		<img alt="Issues" src="https://img.shields.io/github/issues/catppuccin/catppuccin?style=for-the-badge&logo=bilibili&color=F5E0DC&logoColor=D9E0EE&labelColor=302D41"></a>
	<a href="https://github.com/catppuccin/catppuccin">
		<img alt="Repo Size" src="https://img.shields.io/github/repo-size/catppuccin/catppuccin?color=%23DDB6F2&label=SIZE&logo=codesandbox&style=for-the-badge&logoColor=D9E0EE&labelColor=302D41"/></a>
</p>

&nbsp;

<p align="center">
	A set of software tools by Catppuccin developers, for Catppuccin developers
</p>

&nbsp;

### 🪴 Index

+ [Palette Builder](#-palette-builder)
+ [Circular Thumbnails Generator](#-circular-thumbnails-generator)
+ [InkCat](#-inkcat)
+ [Meow](#-meow)

&nbsp;

&nbsp;

#### 👷 Palette Builder

Export the colors of every Catppuccin flavour into various formats. Currently it supports:
+ CSS
+ Json

Usage:

```bash
$ npm install	# fetch dependencies
$ npm start <format> <out_file>
```

Where:
+ `<format>`: can be `json` or `css`
+`<out_file>`: is an optional parameter. In its absence the new format is printed to the console

&nbsp;

#### 🟠 Circular Thumbnails Generator

Generate circular thumbnails of the palettes

Usage:

```bash
$ python3 -m pip install --upgrade pip
$ python3 -m pip install --upgrade Pillow
$ python3 __init__.py <--show>
```

Where:
+ `--show`: is an optional parameter that displays the PNGs in your device's image previewing software instead saving them into an `out/` directory

&nbsp;

#### 🖌️ InkCat

InkCat is a minimal and versatile solution for displaying colors from each one of Catppuccin's
flavours in your terminal. This program also allows you to copy them to your clipboard.

Usage:

```bash
$ npm install -g @catppuccin/inkcat && inkcat --help	# Install command
$ npm uninstall -g @catppuccin/inkcat	# Uninstall command
```

> Note: use `sudo` if needed

Help info:

```
Usage:
	#1: inkcat <palette(s)> <color(s)>
	#2: inkcat <flags>

Args:
	<palette(s)>            Palettes separated by commas (e.g. frappe,latte)
	<color(s)>              Colors separated by commas (e.g. peach,rosewater,lavender)

Flags:
	-h,--help               See this help message
	-n,--no-copy            Don't copy the colors to the clipboard
	-f,--format             Specify format (hex, hsl, rgb)

Exmaples:
	#1: inkcat frappe,latte peach,rosewater,lavender
	#2: inkcat macchiato base,yellow --no-copy
	#3: inkcat mocha
	#4: inkcat mocha maroon --format rgb
```

Testing:

```bash
$ npm install	# fetch dependencies
$ make link	# to link the binary locally
$ make unlink	# to unlink the binary
```

> Note: use `sudo` if needed

&nbsp;

#### 🌈 Contrast Test

Test Catppuccin's flavours compliance with modern web contrast standards

Usage:

```bash
$ npm insatll	# fetch dependencies
$ npm start
```

&nbsp;

#### 🇨🇭 DocPuccin

Fetch health files needed per project type

&nbsp;

#### 😽 Meow

"Waouh Waouh", said the French Poodle

&nbsp;

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/dev/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2021-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
<p align="center"><a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=302d41&colorB=c9cbff"/></a></p>

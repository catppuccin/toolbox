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

+ [👷 Palette Builder](#-palette-builder)
+ [🟠 Circular Thumbnails Generator](#-circular-thumbnails-generator)
+ [🖌️ InkCat](#-inkcat)
+ [🇨 DocPuccin](#-docpuccin)
+ [🖼️ Puccinier](#-puccinier)
+ [🌈 Contrast Test](#-contrast-test)
+ [😽 Meow](#-meow)

&nbsp;

&nbsp;

#### 👷 Palette Builder

Export the colors of every Catppuccin flavour into various formats. Currently it supports:
+ CSS
+ Json

Usage:

```bash
$ git clone https://github.com/catppuccin/toolbox.git
$ cd toolbox/palette_builder
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

#### 🇨🇭 DocPuccin

Docpuccin is a small program that fetches health files needed per project type

Usage:

```bash
$ npm install -g @catppuccin/docpuccin && docpuccin --help	# Install command
$ npm uninstall -g @catppuccin/docpuccin	# Uninstall command
```

> Note: use `sudo` if needed

Help info:

```
Usage:
	#1: docpuccin <health_file_type> <file(s)>
	#2: docpuccin <flags>

Args:
	<health_file_type>	Check the "Available health files" section
	<file(s)>		Health files to be downloaded. They can be comma separated (e.g. npmrc,npmignore)

Flags:
	-h,--help		See this help message

Exmaples:
	#1: docpuccin npm npmignore
	#2: docpuccin repo license
	#3: docpuccin any makefile,editorconfig`
```

To see the available health files please download the tool and run it with the `--help` flag.

Testing:

```bash
$ git clone https://github.com/catppuccin/toolbox.git && cd toolbox/docpuccin/
$ npm install	# fetch dependencies
$ make link	# to link the binary locally
$ make unlink	# to unlink the binary
```

> Note: use `sudo` if needed

&nbsp;

#### 🖼️ Puccinier

This is a small program that helps you generate the rest of the Catppuccin palettes off of a template file written in one of them.

Help info:

```
Usage:
	#1: ${PROGRAM}  <file(s)> <flags>

Args:
	<file(s)>\tFiles to be converted to other palettes. They can be comma separated (e.g. hello.txt,something.json)

Flags:
	-o,--out\tPalettes to export. One may specify many, just separete them with commas (e.g. --out frappe,latte)
	-s,--source\tSpecify the source palette of the files (if none is given, it will be detected)
	-v,--verbose\tEnables verbosity
	-h,--help\tSee this help message

Exmaples:
	#1: ${PROGRAM} project.json --out frappe
	#2: ${PROGRAM} --out latte,macchiato project.json,hello.txt,some_file
	#3: ${PROGRAM} --source frappe --out mocha --verbose my_file.md`
```

Usage:

```bash
$ npm install -g @catppuccin/puccinier && puccinier --help	# Install command
$ npm uninstall -g @catppuccin/puccinier	# Uninstall command
```

> Note: use `sudo` if needed

Testing:

```bash
$ git clone https://github.com/catppuccin/toolbox.git && cd toolbox/puccinier/
$ npm install	# fetch dependencies
$ make link	# link the binary locally
$ make unlink	# unlink the binary
```

> Note: use `sudo` if needed

&nbsp;

#### 🌈 Contrast Test

Test Catppuccin's flavours compliance with modern web contrast standards

Usage:

```bash
$ git clone https://github.com/catppuccin/toolbox.git && cd toolbox/contrast_test/
$ npm install	# fetch dependencies
$ npm start
```

&nbsp;

#### 😽 Meow

"Waouh Waouh", said the French Poodle

&nbsp;

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2021-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
<p align="center"><a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=302d41&colorB=b7bdf8"/></a></p>

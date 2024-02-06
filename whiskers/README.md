<h3 align="center">
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
  Catppuccin Whiskers
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
  <a href="https://github.com/catppuccin/toolbox/stargazers"><img src="https://img.shields.io/github/stars/catppuccin/toolbox?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/toolbox/issues?label=whiskers"><img src="https://img.shields.io/github/issues/catppuccin/toolbox?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/toolbox/contributors"><img src="https://img.shields.io/github/contributors/catppuccin/toolbox?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

&nbsp;

A templating tool to simplify the creation of Catppuccin ports.

## Installation

### Cargo

#### Crates.io

```console
$ cargo install catppuccin-whiskers
```

#### From source

```console
$ cargo install --git https://github.com/catppuccin/toolbox whiskers
```

### Download binaries

[Compiled binaries are available for Windows, macOS, and Linux.](https://github.com/catppuccin/toolbox/releases)

Download the correct file for your system and place it somewhere in your executable path.

### Homebrew

```console
$ brew install catppuccin/tap/whiskers
```

### Nix

```console
$ nix run github:catppuccin/toolbox#whiskers -- <template> <flavor>
```

## Usage

Make a template per file type that your port requires, then use the Whiskers CLI to populate the colors for all four Catppuccin flavors.

```console
$ whiskers <template> <flavor>
```

```console
$ whiskers --help
Soothing port creation tool for the high-spirited!

Usage: whiskers [OPTIONS] [TEMPLATE] [FLAVOR]

Arguments:
  [TEMPLATE]  Path to the template file to render, or `-` for stdin
  [FLAVOR]    Flavor to get colors from [possible values: latte, frappe, macchiato, mocha]

Options:
      --override <OVERRIDES>       The overrides to apply to the template in key=value format
  -o, --output-path <OUTPUT_PATH>  Path to write to instead of stdout
      --check <CHECK>              Instead of printing a result just check if anything would change
  -l, --list-helpers               List all template helpers in markdown format
  -h, --help                       Print help
  -V, --version                    Print version
```

## Template Syntax

Templates are written in [Handlebars](https://handlebarsjs.com/guide/expressions.html) syntax.

### Context Variables

The following variables are available for use in your templates:

- `flavor` (string): The name of the flavor being templated. Possible values: `latte`, `frappé`, `macchiato`, `mocha`.
- `isLight` (bool): True if `flavor` is `latte`, false otherwise.
- `isDark` (bool): True unless `flavor` is `latte`.
- All named colors in the flavor, such as `red`, `subtext0`, and `crust`. A full list of named colors can be found [here](https://github.com/catppuccin/rust/blob/5124eb99eb98d7111dca24537d428a6078e5bbb6/src/flavour.rs#L41-L66). Each color is formatted as hex by default.
- All frontmatter variables as described in the [Frontmatter](#frontmatter) section.

### Helpers

The following custom helpers are available:

- `uppercase string` : Convert a string to uppercase.
  - `{{ uppercase "hello" }}` → `HELLO`
- `lowercase string` : Convert a string to lowercase.
  - `{{ lowercase "HELLO" }}` → `hello`
- `titlecase string` : Convert a string to titlecase.
  - `{{ titlecase "hello there" }}` → `Hello There`
- `trunc number places` : Format a number to a string with a given number of places.
  - `{{ trunc 3.14159265 2 }}` → `3.14`
- `lighten color amount` : Lighten a color by a percentage.
  - `{{ lighten red 0.1 }}` → `f8bacc` / `hsl(343, 81%, 85%)`
- `darken color amount` : Darken a color by a percentage.
  - `{{ darken red 0.1 }}` → `ee5c85` / `hsl(343, 81%, 65%)`
- `mix color_a color_b ratio` : Mix two colors together in a given ratio.
  - `{{ mix red base 0.3 }}` → `5e4054` (30% red, 70% base)
- `opacity color amount` : Set the opacity of a color.
  - `{{ opacity red 0.5 }}` → `hsla(343, 81%, 75%, 0.50)`
- `unquote value` : Marks a value to be unquoted. Mostly useful for maintaining JSON syntax highlighting in template files when a non-string value is needed.
  - `{{ unquote isLight true }}` → `true` (the surrounding quotation marks have been removed)
- `rgb color` : Convert a color to CSS RGB format.
  - `{{ rgb red }}` → `rgb(243, 139, 168)`
- `rgba color` : Convert a color to CSS RGBA format.
  - `{{ rgba (opacity red 0.6) }}` → `rgba(243, 139, 168, 0.60)`
- `hsl color` : Convert a color to CSS HSL format.
  - `{{ hsl red }}` → `hsl(343, 81%, 75%)`
- `hsla color` : Convert a color to CSS HSLA format.
  - `{{ hsla (opacity red 0.6) }}` → `hsla(343, 81%, 75%, 0.60)`
- `red_i color` : Get the red channel of a color as an integer from 0 to 255.
  - `{{ red_i red }}` → `243`
- `green_i color` : Get the green channel of a color as an integer from 0 to 255.
  - `{{ green_i red }}` → `139`
- `blue_i color` : Get the blue channel of a color as an integer from 0 to 255.
  - `{{ blue_i red }}` → `168`
- `alpha_i color` : Get the alpha channel of a color as an integer from 0 to 255.
  - `{{ alpha_i (opacity red 0.6) }}` → `153`
- `red_f color` : Get the red channel of a color as a float from 0 to 1.
  - `{{ red_f red }}` → `0.95` (truncated to 2 places)
- `green_f color` : Get the green channel of a color as a float from 0 to 1.
  - `{{ green_f red }}` → `0.55` (truncated to 2 places)
- `blue_f color` : Get the blue channel of a color as a float from 0 to 1.
  - `{{ blue_f red }}` → `0.66` (truncated to 2 places)
- `alpha_f color` : Get the alpha channel of a color as a float from 0 to 1.
  - `{{ alpha_f (opacity red 0.6) }}` → `0.60` (truncated to 2 places)
- `darklight if-dark if-light` : Choose a value depending on the current flavor. Latte is light, while Frappé, Macchiato, and Mocha are all dark.
  - `{{ darklight "Night" "Day" }}` → `Day` on Latte, `Night` on other flavors

## Frontmatter

You can include additional context variables in the templating process by adding it to an optional YAML frontmatter section at the top of your template file.

As a simple example, given the following template (`example.cfg`):

```handlebars
--- app: 'Pepperjack' author: 'winston' --- # Catppuccin for
{{app}}
# by
{{author}}
bg = '{{base}}' fg = '{{text}}'
```

Running `whiskers example.cfg mocha` produces the following output:

```ini
# Catppuccin for Pepperjack
# by winston
bg = '1e1e2e'
fg = 'cdd6f4'
```

Values in YAML frontmatter are rendered in the same way as the rest of the template, which means you can also make use of context variables in your frontmatter. This can be useful for things like setting an accent color:

```handlebars
--- accent: "{{mauve}}" darkGreen: "{{darken green 0.3}}" --- bg = "#{{base}}"
fg = "#{{text}}" border = "#{{accent}}" diffAddFg = "#{{green}}" diffAddBg = "#{{darkGreen}}"
```

Rendering the above template produces the following output:

```ini
bg = "#1e1e2e"
fg = "#cdd6f4"
border = "#cba6f7"
diffaddfg = "#a6e3a1"
diffaddbg = "#40b436"
```

## Overrides

Whiskers supports overriding individual template values without changing the underlying template source. To use this feature, pass the `--override` flag to the whiskers CLI. You can use the `--override` flag multiple times to apply multiple overrides.

We'll use the following template for this example:

```handlebars
--- accent: '{{mauve}}' --- bg = "#{{base}}" fg = "#{{accent}}"
```

With no overrides passed to whiskers, we get the following output:

```ini
bg = "#1e1e2e"
fg = "#cba6f7"
```

However, we can pass overrides through the CLI with `--override accent=40b436`. Then, we get:

```ini
bg = "#1e1e2e"
fg = "#40b436"
```

We can also override with another value from the [template context](context-variables), for example `--override accent=sky`. This gives the following result:

```ini
bg = "#1e1e2e"
fg = "#89dceb"
```

Finally, we can override both values by passing two overrides. If we invoke whiskers with `--override accent=yellow --override base=000000` then we get this output:

```ini
bg = "#000000"
fg = "#f9e2af"
```

## Check Mode

You can use Whiskers as a linter with _check mode_. To do so, set the `--check` option to a file containing the expected output. Whiskers will render your template as per usual, but then instead of printing the result it will check it against the expected output and fail with exit code 1 if they differ.

This is especially useful in CI pipelines to ensure that the generated files are not changed without a corresponding change to the templates.

Whiskers will diff the output against the check file using the program set in the `DIFFTOOL` environment variable, falling back to `diff` if it's not set. The command will be invoked as `$DIFFTOOL <actual> <expected>`.

```console
$ whiskers theme.hbs latte --check themes/latte.cfg
(no output, exit code 0)

$ whiskers theme.hbs latte --check themes/latte.cfg
Templating would result in changes.
4c4
< accent is #ea76cb
---
> accent is #40a02b

(exit code 1)
```

## Further Reading

- [The example template](examples/example.hbs) demonstrates the template syntax and usage of some of the helpers.
- Some real ports that use Whiskers:
  - [Qt Creator](https://github.com/catppuccin/qtcreator)
  - [Dwarf Fortress](https://github.com/catppuccin/dwarf-fortress)

&nbsp;

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
<p align="center"><a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=302d41&colorB=b7bdf8"/></a></p>

<h3 align="center">
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/logos/exports/1544x1544_circle.png" width="100" alt="Logo"/><br/>
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
  Catppuccin Whiskers
  <img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/misc/transparent.png" height="30" width="0px"/>
</h3>

<p align="center">
  <a href="https://github.com/catppuccin/toolbox/stargazers"><img src="https://img.shields.io/github/stars/catppuccin/toolbox?colorA=363a4f&colorB=b7bdf8&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/toolbox/issues"><img src="https://img.shields.io/github/issues/catppuccin/toolbox?colorA=363a4f&colorB=f5a97f&style=for-the-badge"></a>
  <a href="https://github.com/catppuccin/toolbox/contributors"><img src="https://img.shields.io/github/contributors/catppuccin/toolbox?colorA=363a4f&colorB=a6da95&style=for-the-badge"></a>
</p>

&nbsp;

A templating tool to simplify the creation of Catppuccin ports.

## Installation

You can install the binary from using one of the methods below:

| Installation Method                   | Instructions                                                                                                        |
|---------------------------------------|---------------------------------------------------------------------------------------------------------------------|
| crates.io                             | `cargo install catppuccin-whiskers`                                                                                 |
| Source                                | `cargo install --git https://github.com/catppuccin/toolbox whiskers`                                                |
| Homebrew                              | `brew install catppuccin/tap/whiskers`                                                                              |
| Nix                                   | `nix profile install github:catppuccin/toolbox#whiskers`<br/>`nix run github:catppuccin/toolbox#whiskers -- <args>` | 
| Binaries<br/>(Windows, MacOS & Linux) | Download the [latest GitHub release](https://github.com/catppuccin/toolbox/releases/latest)                         |

## Usage

Whiskers is a port creation helper tool that is custom-built for Catppuccin,
allowing developers to define template files which the palette can be injected
into.

```console
$ whiskers --help
Soothing port creation tool for the high-spirited!

Usage: whiskers [OPTIONS] [TEMPLATE] [FLAVOR]

Arguments:
  [TEMPLATE]  Path to the template file to render, or `-` for stdin
  [FLAVOR]    Flavor to get colors from [possible values: latte, frappe, macchiato, mocha, all]

Options:
      --overrides <OVERRIDES>      The overrides to apply to the template in JSON format
  -o, --output-path <OUTPUT_PATH>  Path to write to instead of stdout
      --check <CHECK>              Instead of printing a result, check if anything would change
  -l, --list-helpers               List all template helpers in Markdown format
  -h, --help                       Print help
  -V, --version                    Print version
```

## Template

Please familiarize yourself with [Handlebars](https://handlebarsjs.com/guide/),
which is the templating engine used in whiskers.

### Context Variables

The following variables are available for use in your templates:

| Variable                                                                                                                                                    | Description                                                                                                                                                                 |
|-------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `flavor` (string)                                                                                                                                           | The name of the flavor being templated. Possible values: `latte`, `frappé`, `macchiato`, `mocha`.                                                                           | 
| `isLight` (bool)                                                                                                                                            | True if `flavor` is `latte`, false otherwise.                                                                                                                               |
| `isDark` (bool)                                                                                                                                             | True unless `flavor` is `latte`.                                                                                                                                            |
| `rosewater`, `flamingo`, `pink`, [(etc.)](https://github.com/catppuccin/rust/blob/5124eb99eb98d7111dca24537d428a6078e5bbb6/src/flavour.rs#L41-L66) (string) | All named colors in each flavor, each color is formatted as hex by default.                                                                                                 | 
| `colors` (array)                                                                                                                                            | An array containing all of the named colors.                                                                                                                                |
| `flavors` (array)                                                                                                                                           | An array containing all of the named flavors, with every other context variable.<br/><strong>See [Single File Support](#Single-File-Support) for more information.</strong> |
| Any Frontmatter                                                                                                                                             | All frontmatter variables as described in the [Frontmatter](#Frontmatter) section.                                                                                          |

### Helpers

The following custom helpers are available:

| Helper<br/>(`<>` values are args)     | Input                             | Output                                                     | Description                                                                                                                               |
|---------------------------------------|-----------------------------------|------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| uppercase \<string\>                  | `{{ uppercase "hello" }}`         | `HELLO`                                                    | Convert a string to uppercase.                                                                                                            |
| lowercase \<string\>                  | `{{ lowercase "HELLO" }}`         | `hello`                                                    | Convert a string to lowercase.                                                                                                            |
| titlecase \<string\>                  | `{{ titlecase "hello there" }}`   | `Hello There`                                              | Convert a string to titlecase.                                                                                                            |
| trunc \<number\> \<places\>           | `{{ trunc 3.14159265 2 }}`        | `3.14`                                                     | Format a number to a string with a given number of places.                                                                                |
| lighten \<color\> \<amount\>          | `{{ lighten red 0.1 }}`           | `f8bacc` / `hsl(343, 81%, 85%)`                            | Lighten a color by a percentage.                                                                                                          |
| darken \<color\> \<amount\>           | `{{ darken red 0.1 }}`            | `ee5c85` / `hsl(343, 81%, 65%)`                            | Darken a color by a percentage.                                                                                                           |
| mix \<color_a\> \<color_b\> \<ratio\> | `{{ mix red base 0.3 }}`          | `5e4054` (30% red, 70% base)                               | Mix two colors together in a given ratio.                                                                                                 |
| opacity \<color\> \<amount\>          | `{{ opacity red 0.5 }}`           | `hsla(343, 81%, 75%, 0.50)`                                | Set the opacity of a color.                                                                                                               |
| unquote \<value\>                     | `"{{ unquote isLight true }}"`    | `true` (the surrounding quotation marks have been removed) | Marks a value to be unquoted. Mostly useful for maintaining JSON syntax highlighting in template files when a non-string value is needed. |
| rgb \<color\>                         | `{{ rgb red }}`                   | `rgb(243, 139, 168)`                                       | Convert a color to CSS RGB format.                                                                                                        |
| rgba \<color\>                        | `{{ rgba (opacity red 0.6) }}`    | `rgba(243, 139, 168, 0.60)`                                | Convert a color to CSS RGBA format.                                                                                                       |
| hsl \<color\>                         | `{{ hsl red }}`                   | `hsl(343, 81%, 75%)`                                       | Convert a color to CSS HSL format.                                                                                                        |
| hsla \<color\>                        | `{{ hsla (opacity red 0.6) }}`    | `hsla(343, 81%, 75%, 0.60)`                                | Convert a color to CSS HSLA format.                                                                                                       |
| red_i \<color\>                       | `{{ red_i red }}`                 | `243`                                                      | Get the red channel of a color as an integer from 0 to 255.                                                                               |
| green_i \<color\>                     | `{{ green_i red }}`               | `139`                                                      | Get the green channel of a color as an integer from 0 to 255.                                                                             |
| blue_i \<color\>                      | `{{ blue_i red }}`                | `168`                                                      | Get the blue channel of a color as an integer from 0 to 255.                                                                              |
| alpha_i \<color\>                     | `{{ alpha_i (opacity red 0.6) }}` | `153`                                                      | Get the alpha channel of a color as an integer from 0 to 255.                                                                             |
| red_f \<color\>                       | `{{ red_f red }}`                 | `0.95` (truncated to 2 places)                             | Get the red channel of a color as a float from 0 to 1.                                                                                    |
| green_f \<color\>                     | `{{ green_f red }}`               | `0.55` (truncated to 2 places)                             | Get the green channel of a color as a float from 0 to 1.                                                                                  |
| blue_f \<color\>                      | `{{ blue_f red }}`                | `0.66` (truncated to 2 places)                             | Get the blue channel of a color as a float from 0 to 1.                                                                                   |
| alpha_f \<color\>                     | `{{ alpha_f (opacity red 0.6) }}` | `0.60` (truncated to 2 places)                             | Get the alpha channel of a color as a float from 0 to 1.                                                                                  |
| red_h \<color\>                       | `{{ red_h red }}`                 | `f3`                                                       | Get the red channel of a color as a hexadecimal number from 00 to ff.                                                                     |
| green_h \<color\>                     | `{{ green_h red }}`               | `8b`                                                       | Get the green channel of a color as a hexadecimal number from 00 to ff.                                                                   |
| blue_h \<color\>                      | `{{ blue_h red }}`                | `a8`                                                       | Get the blue channel of a color as a hexadecimal number from 00 to ff.                                                                    |
| alpha_h \<color\>                     | `{{ alpha_h (opacity red 0.6) }}` | `99`                                                       | Get the alpha channel of a color as a hexadecimal number from 00 to ff.                                                                   |
| darklight \<if-dark\> \<if-light\>    | `{{ darklight "Night" "Day" }}`   | `Day` on Latte, `Night` on other flavors                   | Choose a value depending on the current flavor. Latte is light, while Frappé, Macchiato, and Mocha are all dark.                          |

## Frontmatter

You can include additional context variables in the templating process by adding
it to an optional YAML frontmatter section at the top of your template file.

As a simple example, given the following template (`example.cfg`):

```yaml
---
app: 'Pepperjack'
author: 'winston'
---
# Catppuccin for {{app}}
# by {{author}}
bg = '{{base}}'
fg = '{{text}}'
```

Running `whiskers example.cfg mocha` produces the following output:

```yaml
# Catppuccin for Pepperjack
# by winston
bg = '1e1e2e'
fg = 'cdd6f4'
```

Values in YAML frontmatter are rendered in the same way as the rest of the
template, which means you can also make use of context variables in your
frontmatter. This can be useful for things like setting an accent color:

```yaml
---
accent: "{{mauve}}"
darkGreen: "{{darken green 0.3}}"
---
bg = "#{{base}}"
fg = "#{{text}}"
border = "#{{accent}}"
diffAddFg = "#{{green}}"
diffAddBg = "#{{darkGreen}}"
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

### Frontmatter

Whiskers supports overriding template values in the frontmatter itself. For
example, this can be useful for changing variables depending on the flavor:

`example.yml`

```yaml
---
accent: "{{mauve}}"
overrides:
  latte: # only applies to Latte
    accent: "{{pink}}"
  mocha: # only applies to Mocha
    accent: "{{blue}}"
---
{{flavor}} has accent color {{accent}}.
```

When running `whiskers example.yml {latte, frappe, macchiato, mocha}`, we see that:

- Frappe & Macchiato will have the accent `mauve` hex code.
- Latte will have the accent `pink` hex code.
- Mocha will have the accent `blue` hex code.

### CLI

Overrides can also be specified through the cli via the `--overrides` flag, taking in a JSON string resembling the
frontmatter. This is particularly useful with build scripts to automatically generate files for each accent:

`example.yml`

```yaml
---
accent: "{{mauve}}"
---
theme:
  accent: "{{accent}}"
```

When running `whiskers example.yml latte --overrides '{"accent": "{{pink}}"}'`,
the `accent` will be overridden to pink.

### Frontmatter & CLI

Overrides can be specified both in the frontmatter and the CLI but it is
important to understand the order of priority:

1. CLI overrides (`--overrides` flag.)
2. Frontmatter `overrides` block.
3. Frontmatter root context.

To express this visually, given an `example.yml` file:

```yaml
---
accent: "{{mauve}}"     # <-- Frontmatter Root Context
background: "{{base}}"
text: "{{text}}"
overrides: # <-- Frontmatter Overrides Block
  mocha:
    accent: "{{blue}}"
---
```

and the command:

```shell
whiskers example.yml mocha --overrides '{"accent": "{{pink}}"}' # <-- CLI Overrides
```

The resulting file will have the accent `pink` as the accent will go through the
following transformations:

1. accent is set to `mauve` in the root context.
2. accent is overridden to `blue` in the overrides block.
3. accent is overridden again to `pink` in the CLI overrides.

## Single File Support

Sometimes, you may not want to generate a file per flavor, but rather use all
the flavors inside one single file. This is achieved specifying the `<template>`
argument as `all`. (e.g. `whiskers example.yml all`)

When the `<template>` has been set to `all`, a new context variable `flavors` is
available which can be iterated through the `{{#each}}` handlebars helper. E.g.
if we have the following contexts:

`latte`

```json
{
  "flavor": "latte",
  "isLight": true,
  "isDark": false,
  "rosewater": "#dc8a78",
  ...
}
```

`frappe`

```json
{
  "flavor": "frappe",
  "isLight": false,
  "isDark": true,
  "rosewater": "#f2d5cf",
  ...
}
```

`macchiato`

```json
{
  "flavor": "macchiato",
  "isLight": false,
  "isDark": true,
  "rosewater": "#f4dbd6",
  ...
}
```

`mocha`

```json
{
  "flavor": "mocha",
  "isLight": false,
  "isDark": true,
  "rosewater": "#f5e0dc",
  ...
}
```

The `all` context looks like the following:

```json
{
  "flavors": {
    "latte": {
      "flavor": "latte",
      "isLight": true,
      "isDark": false,
      "rosewater": "#dc8a78",
      ...
    },
    "frappe": {
      "flavor": "frappe",
      "isLight": false,
      "isDark": true,
      "rosewater": "#f2d5cf",
      ...
    },
    "macchiato": {
      "flavor": "macchiato",
      "isLight": false,
      "isDark": true,
      "rosewater": "#f4dbd6",
      ...
    },
    "mocha": {
      "flavor": "mocha",
      "isLight": false,
      "isDark": true,
      "rosewater": "#f5e0dc",
      ...
    }
  }
}
```

This allows us to define a template file like the following:

`input.md`
```md
# Single File

{{#each flavors}}
## {{titlecase flavor}}
Accent: #{{mauve}}
{{/each}}
```

and after running `whiskers input.md all -o README.md`, we get the following output file:

`README.md`
```md
# Single File

## Latte
Accent: #8839ef
## Frappe
Accent: #ca9ee6
## Macchiato
Accent: #c6a0f6
## Mocha
Accent: #cba6f7
```

Please see the [examples/single-file](examples/single-file) directory for more
concrete examples on how it can be used.

## Check Mode

You can use Whiskers as a linter with *check mode*. To do so, set the `--check`
option to a file containing the expected output. Whiskers will render your
template as per usual, but then instead of printing the result it will check it
against the expected output and fail with exit code 1 if they differ.

This is especially useful in CI pipelines to ensure that the generated files are
not changed without a corresponding change to the templates.

Whiskers will diff the output against the check file using the program set in
the `DIFFTOOL` environment variable, falling back to `diff` if it's not set. The
command will be invoked as `$DIFFTOOL <actual> <expected>`.

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

- See the [examples](examples) directory which further showcase the utilities
  and power of whiskers.
- See the draft RFC,
  [CAT-0003-Whiskers](https://github.com/catppuccin/community/pull/12), to
  understand the motivations behind creating whiskers.

&nbsp;

<p align="center"><img src="https://raw.githubusercontent.com/catppuccin/catppuccin/main/assets/footers/gray0_ctp_on_line.svg?sanitize=true" /></p>
<p align="center">Copyright &copy; 2023-present <a href="https://github.com/catppuccin" target="_blank">Catppuccin Org</a>
<p align="center"><a href="https://github.com/catppuccin/catppuccin/blob/main/LICENSE"><img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=302d41&colorB=b7bdf8"/></a></p>

#!/usr/bin/env node

import { variants, labels } from '@catppuccin/palette'
import clipboard from 'clipboardy'
import chalk from 'chalk'

const BLACK = variants['latte']['text']['hex']
const WHITE = variants['latte']['base']['hex']
let no_copy = false
let final_colors = ''
let format = 'hex'
const argv = process.argv.slice(2)
const HELP = `InkCat is a minimal and versatile solution for displaying colors from each one of Catppuccin's
flavours in your terminal. This program also allows you to copy them to your clipboard.

Usage:
	#1: inkcat <palette(s)> <color(s)>
	#2: inkcat <flags>

Args:
	<palette(s)>\t\tPalettes separated by commas (e.g. frappe,latte)
	<color(s)>\t\tColors separated by commas (e.g. peach,rosewater,lavender)

Flags:
	-h,--help\t\tSee this help message
	-n,--no-copy\t\tDon't copy the colors to the clipboard
	-f,--format\t\tSpecify format (hex, hsl, rgb)

Examples:
	#1: inkcat frappe,latte peach,rosewater,lavender
	#2: inkcat macchiato base,yellow --no-copy
	#3: inkcat mocha
	#4: inkcat mocha maroon --format rgb`

function append(str) {
    final_colors = final_colors + ',' + str
}

function write(msg) {
    process.stdout.write(msg)
}

function abort(msg) {
    console.error(msg)
    process.exit(1)
}

let argv_cp = argv.map((x) => x)
for (let arg in argv_cp) {
    switch (argv_cp[arg]) {
        case '--help':
        case '-h':
            write(HELP + '\n')
            process.exit(0)
        case '--no-copy':
        case '-n':
            no_copy = true
            argv.splice(arg, 1)
            break
        case '--format':
        case '-f':
            format = argv_cp[parseInt(arg) + 1]
            if (format == undefined) {
                abort(
                    'You must provide a color format after the `--format` flag'
                )
            }

            if (!['hex', 'hsl', 'rgb'].includes(format)) {
                abort(
                    "The format '" +
                        format +
                        "' was not recognized! Check `--help`"
                )
            }
            argv.splice(arg, 1)
            argv.splice(arg + 1, 1)
            break
        default:
            break
    }
}

let picked_palettes
let picked_colors

function parse_args(args) {
    if (args != undefined) {
        return args.split(',')
    }
}

function assert_inclusion(test_arr, original_arr) {
    for (let element in test_arr) {
        if (test_arr[element] == '') {
            test_arr.splice(element, 1)
        } else {
            if (!(test_arr[element] in original_arr)) {
                abort("'" + test_arr[element] + "' was not recognized")
            }
        }
    }
}

if (argv[0] != undefined) {
    picked_palettes = parse_args(argv[0])
    picked_colors = parse_args(argv[1])
} else {
    abort('You must pass at least one flavour name!')
}

assert_inclusion(picked_palettes, variants)
assert_inclusion(picked_colors, labels)

function get_tabs(str) {
    if (str.length > 6) {
        return '\t'
    }
    return '\t\t'
}

// https://stackoverflow.com/questions/1026069/how-do-i-make-the-first-letter-of-a-string-uppercase-in-javascript
function capitalize(string) {
    return string.charAt(0).toUpperCase() + string.slice(1)
}

function get_rgb(c) {
    return parseInt(c, 16) || c
}

function gets_rgb(c) {
    return get_rgb(c) / 255 <= 0.03928
        ? get_rgb(c) / 255 / 12.92
        : Math.pow((get_rgb(c) / 255 + 0.055) / 1.055, 2.4)
}

function get_luminance(hex_color) {
    return (
        0.2126 * gets_rgb(hex_color.substr(1, 2)) +
        0.7152 * gets_rgb(hex_color.substr(3, 2)) +
        0.0722 * gets_rgb(hex_color.substr(-2))
    )
}

function get_contrast(f, b) {
    const L1 = get_luminance(f)
    const L2 = get_luminance(b)
    return (Math.max(L1, L2) + 0.05) / (Math.min(L1, L2) + 0.05)
}

function print_color(text, background, preferred_white, preffered_black) {
    write(
        chalk.bgHex(background)(
            chalk.hex(
                get_text_color(background, preferred_white, preffered_black)
            )(text)
        )
    )
}

function get_text_color(bg_color, preferred_white, preffered_black) {
    let white = preferred_white ? preferred_white : '#ffffff'
    let black = preffered_black ? preffered_black : '#000000'
    const whiteContrast = get_contrast(bg_color, white)
    const blackContrast = get_contrast(bg_color, black)

    return whiteContrast > blackContrast ? white : black
}

if (picked_colors == undefined) {
    for (let palette in picked_palettes) {
        print_color(
            capitalize(picked_palettes[palette]) + '\t\t',
            WHITE,
            BLACK,
            BLACK
        )
    }
    write('\n\n')

    for (let label in labels) {
        for (let palette in picked_palettes) {
            print_color(
                label + get_tabs(label),
                variants[picked_palettes[palette]][label][format]
            )
        }
        write('\n')
    }
} else {
    for (let palette in picked_palettes) {
        for (let color in picked_colors) {
            append(
                labels[picked_colors[color]][picked_palettes[palette]][format]
            )
        }
    }

    final_colors = final_colors.replace(/(^,)|(,$)/g, '')

    if (no_copy == true) {
        write(final_colors)
        process.exit(0)
    }
    clipboard.writeSync(final_colors)
}

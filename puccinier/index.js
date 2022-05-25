#!/usr/bin/env node

import { variants, labels } from '@catppuccin/palette'
import chalk from 'chalk'
import {
	copyFile,
	readFile,
	writeFile,
	writeFileSync,
	promises as fsPromises,
} from 'fs'
import replace from 'replace'

const argv = process.argv.slice(2)
let source
let out_palettes
let verbose = false
const PROGRAM = "puccinier"
const HELP = `Generate the rest of the Catppuccin palettes off of a template file written in one of them.

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

function write(msg) {
	process.stdout.write(msg)
}

function abort(msg) {
	console.error(msg)
	process.exit(1)
}

function parse_args(args) {
	if (args != undefined) {
		return args.split(',')
	}
}

function pop(val) {
	argv.splice(argv.indexOf(val), 1)
}

function print(str, color = 'blue', error) {
	if (error) {
		console.error(chalk.hex(variants['macchiato'][color]['hex'])(str))
		return
	}
	console.log(chalk.hex(variants['macchiato'][color]['hex'])(str))
}

let argv_cp = argv.map((x) => x)
for (let arg in argv_cp) {
	switch (argv_cp[arg]) {
		case '--help':
		case '-h':
			write(HELP + '\n')
			process.exit(0)
		case '--source':
		case '-s':
			source = argv_cp[parseInt(arg) + 1]
			pop(argv_cp[arg])
			if (source == undefined) {
				abort(
					"Place specify the source palette to pick up colors from after the '--source' flag"
				)
			}
			pop(argv_cp[parseInt(arg) + 1])
			break
		case '--out':
		case '-o':
			let palettes = argv_cp[parseInt(arg) + 1]
			pop(argv_cp[arg])
			if (palettes != undefined) {
				out_palettes = parse_args(palettes)
				pop(argv_cp[parseInt(arg) + 1])
			} else {
				abort(
					"Please specify the output palettes after the '--out' flag"
				)
			}
			break
		case '--verbose':
		case '-v':
			verbose = true
			pop(argv_cp[arg])
			break
		default:
			break
	}
}

function detect_palette(file) {
	for (let palette in variants) {
		for (let label in labels) {
			if (file.includes(variants[palette][label]['hex'])) {
				return palette
			}
		}
	}
	return 'macchiato' // default
}

let files = parse_args(argv[0])

if (files == undefined || files.length == 0) {
	abort("Please provide files to convert.\nCheck '--help' for examples :)")
}

for (let f in files) {
	print("Processing " + files[f] + "...")
	readFile(files[f], 'utf-8', function (err, contents) {
		if (err) {
			console.log(err)
			return
		}

		if (source == undefined) {
			source = detect_palette(contents)
		}

		print('Detected source palette: ' + source, 'peach')

		for (let palette in out_palettes) {
			let file_extension = files[f].split('.').pop()
			let out_file =
				files[f].replace(/\.[^/.]+$/, '') +
				'_' +
				out_palettes[palette] +
				(file_extension == files[f] ? '' : '.' + file_extension)

			copyFile(files[f], out_file, (err) => {
				if (err) throw err
				print('Created ' + out_file)
			})

			for (let label in labels) {
				if (verbose == true) {
					print("  • Replacing '" + label + "'", 'green')
				}
				replace({
					regex: new RegExp(variants[source][label]['hex'], 'g'),
					replacement: variants[out_palettes[palette]][label]['hex'],
					paths: [out_file],
					recursive: false,
					silent: true,
				})
			}
		}
	})
}

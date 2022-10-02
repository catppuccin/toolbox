#!/usr/bin/env node

import {variants, labels} from '@catppuccin/palette'
import * as https from 'https'
import chalk from 'chalk'
import * as fs from 'fs'

const argv = process.argv.slice(2)
const HEALTH_FILES = {
	repo: {
		coc: 'https://raw.githubusercontent.com/catppuccin/.github/main/CODE_OF_CONDUCT.md',
		license:
			'https://raw.githubusercontent.com/catppuccin/catppuccin/main/LICENSE',
	},
	any: {
		editorconfig:
			'https://raw.githubusercontent.com/catppuccin/.github/main/health/any/.editorconfig',
		prettierrc:
			'https://raw.githubusercontent.com/catppuccin/.github/main/health/any/.prettierrc.json',
		makefile:
			'https://raw.githubusercontent.com/catppuccin/.github/main/health/any/Makefile'
	},
	npm: {
		npmignore:
			'https://raw.githubusercontent.com/catppuccin/.github/main/health/npm/.npmignore',
		npmrc: 'https://raw.githubusercontent.com/catppuccin/.github/main/health/npm/.npmrc',
	},
}

const HELP = `Docpuccin is a small program that fetches health files needed per project type

Usage:
	#1: docpuccin <health_file_type> <file(s)>
	#2: docpuccin <flags>

Args:
	<health_file_type>\tCheck the "Available health files" section
	<file(s)>\t\tHealth files to be downloaded. They can be comma separated (e.g. npmrc,npmignore)

Flags:
	-h,--help\t\tSee this help message

Available health files:

${chalk.hex(variants["macchiato"]["green"]["hex"])(JSON.stringify(HEALTH_FILES, undefined, 2))}

Exmaples:
	#1: docpuccin npm npmignore
	#2: docpuccin repo license
	#3: docpuccin any makefile,editorconfig`

function write(msg) {
    process.stdout.write(msg)
}

function abort(msg) {
	console.error(msg)
	process.exit(1)
}

function parse_args(args) {
	if (args != undefined) {
		return args.split(",")
	}
}

for (let arg in argv) {
	switch (argv[arg]) {
		case "--help":
		case "-h":
			write(HELP + "\n")
			process.exit(0)
		default:
			break;
	}
}

function truncate_filename(file) {
	return file.split('/').pop()
}

function fetch_file(out, url) {
	console.log(chalk.hex(variants["macchiato"]["blue"]["hex"])("Downloading " + out + "..."))
	https.get(url, (res) => {
		const writeStream = fs.createWriteStream(out)
		res.pipe(writeStream)

		writeStream.on('finish', () => {
			writeStream.close()
			console.log(chalk.hex(variants["macchiato"]["green"]["hex"])("  + Downloaded " + out))
		})
	})
}

if (argv[0] == undefined) {
	abort("You must pass a health file identifier!\nTry '--help'")
}

if (!(argv[0] in HEALTH_FILES)) {
	abort("'" + argv[0] + "' is not a valid group of health files!\nTry '--help'")
}

if (argv[1] == undefined) {
	for (let file in HEALTH_FILES[argv[0]]) {
		fetch_file(truncate_filename(HEALTH_FILES[argv[0]][file]), HEALTH_FILES[argv[0]][file])
	}
} else {
	let files = parse_args(argv[1])
	for (let file in files) {
		if (HEALTH_FILES[argv[0]][files[file]] != undefined) {
			fetch_file(truncate_filename(HEALTH_FILES[argv[0]][files[file]]), HEALTH_FILES[argv[0]][files[file]])
		} else {
			console.error(chalk.hex(variants["macchiato"]["red"]["hex"])("The health file '" + files[file] + "' is not in the group '" + argv[0] + "'"))
		}
	}
}

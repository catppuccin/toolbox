mod palette;

use clap::{
    arg, builder::PossibleValuesParser, command, parser::ValuesRef, value_parser, App, ArgMatches,
    ErrorKind,
};
use regex::{Match, Regex};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use palette::{Color, Palette, COLOR_FROM_VARIANT, VARIANT_FROM_COLOR};

fn main() {
    let mut cmd: App = command!()
        .arg(
            arg!(-s --source <FILE> "Set the source file to convert")
                .value_parser(value_parser!(PathBuf))
                .required(true)
        )
        .arg(
            arg!(-o --output <TYPES> "Set the themes (space-separated) to generate from the source file")
                .value_parser(PossibleValuesParser::new(["latte", "frappe", "macchiato", "mocha"]))
                .takes_value(true)
                .multiple_values(true)
                .required(true)
        );

    let matches: ArgMatches = cmd.clone().get_matches();

    let source_path: &PathBuf = match matches.get_one::<PathBuf>("source") {
        Some(path) => path,
        None => cmd
            .error(ErrorKind::ValueValidation, "Failed to read source argument")
            .exit(),
    };

    let source_file: File = match File::options().read(true).write(true).open(source_path) {
        Ok(file) => file,
        Err(e) => cmd
            .error(ErrorKind::Io, format!("Failed to open source file: {e}"))
            .exit(),
    };

    let output_themes: ValuesRef<String> = match matches.get_many("output") {
        Some(output_theme) => output_theme,
        None => cmd
            .error(ErrorKind::ValueValidation, "Failed to read output argument")
            .exit(),
    };

    let mut writers: Vec<BufWriter<File>> = output_themes
        .clone()
        .into_iter()
        .map(|theme: &String| -> BufWriter<File> {
            let extension = match source_path.extension() {
                Some(extension) => extension,
                None => cmd
                    .clone()
                    .error(ErrorKind::Io, "Failed to read source file extension")
                    .exit(),
            };

            BufWriter::new(
                match File::create(source_path.with_file_name(theme).with_extension(extension)) {
                    Ok(file) => file,
                    Err(e) => cmd
                        .clone()
                        .error(ErrorKind::Io, format!("Failed to create output file: {e}"))
                        .exit(),
                },
            )
        })
        .collect();

    lazy_static::lazy_static! {
        static ref HEX: Regex = Regex::new(r"(?i)#([A-F0-9]{6}|[A-F0-9]{3})").unwrap();
        static ref RGB: Regex = Regex::new(r"(?i)rgba?\(.+\)").unwrap();
        static ref HSL: Regex = Regex::new(r"(?i)hsla?\(.+\)").unwrap();
    }

    for line in BufReader::new(&source_file).lines() {
        let line: String = match line {
            Ok(line) => line,
            Err(e) => {
                cmd.clone()
                    .error(ErrorKind::Io, format!("Failed to read line: {e}, skipping"))
                    .print()
                    .unwrap();
                continue;
            }
        };

        let mut items = HEX
            .find_iter(&line)
            .chain(RGB.find_iter(&line))
            .chain(HSL.find_iter(&line))
            .filter_map(|item: Match| -> Option<([&String; 3], &str)> {
                let replacement = item.as_str();

                VARIANT_FROM_COLOR
                    .get(&replacement.to_lowercase())
                    .cloned()
                    .zip(Some(replacement))
            });

        for (i, theme) in output_themes.clone().enumerate() {
            let mut copy: String = line.clone();

            for (lookup, replacement) in items.by_ref() {
                let label: &Palette = COLOR_FROM_VARIANT.get(theme).unwrap();
                let color_format: &Color = label.get(lookup[1]).unwrap();
                let color_value: &String = color_format.get(lookup[2]).unwrap();

                copy = copy.replace(replacement, color_value);
            }

            if let Err(e) = writeln!(writers[i], "{}", copy) {
                cmd.clone()
                    .error(ErrorKind::Io, format!("Failed to write line: {e}"))
                    .print()
                    .unwrap();
            }
        }
    }

    for mut writer in writers {
        if let Err(e) = writer.flush() {
            cmd.clone()
                .error(
                    ErrorKind::Io,
                    format!("Failed to flush writer: {e}. Changes will be dropped."),
                )
                .print()
                .unwrap();
        }
    }
}

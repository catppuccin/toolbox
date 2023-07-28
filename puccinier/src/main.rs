mod palette;

use clap::{arg, builder::PossibleValuesParser, value_parser, ErrorKind};
use regex::{Match, Regex};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use palette::{Color, Palette, COLOR_FROM_VARIANT, VARIANT_FROM_COLOR};

fn main() {
    let mut cmd = clap::command!()
        .arg( arg!(-s --source <FILE> "Set the source file to convert")
			.value_parser(value_parser!(PathBuf))
			.required(true))
        .arg( arg!(-o --output <TYPES> "Set the themes (space-separated) to generate from the source file")
			.value_parser(PossibleValuesParser::new(["latte", "frappe", "macchiato", "mocha"]))
			.takes_value(true)
			.multiple_values(true)
			.required(true));

    let matches = cmd.clone().get_matches();

    let source_path: &PathBuf = match matches.get_one("source") {
        Some(path) => path,
        None => cmd
            .error(ErrorKind::ValueValidation, "Failed to read source argument")
            .exit(),
    };

    let source_file = match File::options().read(true).write(true).open(source_path) {
        Ok(file) => file,
        Err(e) => cmd
            .error(ErrorKind::Io, format!("Failed to open source file: {e}"))
            .exit(),
    };

    let output_themes: Vec<&String> = match matches.get_many::<String>("output") {
        Some(output_themes) => output_themes.collect(),
        None => cmd
            .error(ErrorKind::ValueValidation, "Failed to read output argument")
            .exit(),
    };

    let mut writers: Vec<BufWriter<File>> = output_themes
        .iter()
        .map(|theme| {
            let mut path = source_path.with_file_name(theme);
            if let Some(extension) = source_path.extension() {
                path = path.with_extension(extension);
            }
            BufWriter::new(
                match File::create(path) {
                    Ok(file) => file,
                    Err(e) => cmd
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
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                cmd.error(ErrorKind::Io, format!("Failed to read line: {e}"))
                    .exit();
            }
        };

        for (theme, writer) in output_themes.iter().zip(&mut writers) {
            let mut copy = line.clone();

            HEX.find_iter(&line)
                .chain(RGB.find_iter(&line))
                .chain(HSL.find_iter(&line))
                .filter_map(|item: Match| -> Option<([&String; 3], &str)> {
                    let replacement: &str = item.as_str();

                    VARIANT_FROM_COLOR
                        .get(&replacement.to_lowercase())
                        .cloned()
                        .zip(Some(replacement))
                })
                .for_each(|(lookup, replacement)| {
                    let label: &Palette = COLOR_FROM_VARIANT.get(theme.as_str()).unwrap();
                    let color_format: &Color = label.get(lookup[1]).unwrap();
                    let color_value: &String = color_format.get(lookup[2]).unwrap();

                    copy = copy.replace(replacement, color_value);
                });

            if let Err(e) = writeln!(writer, "{}", copy) {
                cmd.error(ErrorKind::Io, format!("Failed to write line: {e}"))
                    .print()
                    .unwrap();
            }
        }
    }

    for mut writer in writers {
        if let Err(e) = writer.flush() {
            cmd.error(
                ErrorKind::Io,
                format!("Failed to flush writer: {e}. Changes will be dropped."),
            )
            .exit();
        }
    }
}

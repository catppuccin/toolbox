use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

mod palette;

fn main() -> io::Result<()> {
    xflags::xflags! {
        /// Generate the other Catppuccin flavours off a template file written in one of them
        cmd puccinier {
            /// The source file to convert
            required source: PathBuf
            /// Set the themes to generate from the source file (one of 'latte', 'frappe', 'macchiato', or 'mocha')
            repeated -o, --output type: String
        }
    };

    let mut flags = Puccinier::from_env_or_exit();
    flags.output.sort_unstable();
    flags.output.dedup();
    flags.output.retain(|theme| {
		if theme != "latte" && theme != "frappe" && theme != "macchiato" && theme != "mocha" {
			eprintln!("Invalid output theme: {theme}. Must be one of 'latte', 'frappe', 'macchiato', or 'mocha'.");
			eprintln!("Skipping.");
			return false;
		}
		true
	});
    if flags.output.is_empty() {
        eprintln!("Warning: no output themes");
        return Ok(());
    }

    let color_from_variant = palette::palettes();

    let mut variant_from_color: HashMap<&'static str, [&'static str; 3]> = HashMap::new();
    for (variant, labels) in &color_from_variant {
        for (label, colors) in labels.iter() {
            for (format, value) in colors.iter() {
                variant_from_color.insert(value, [variant, label, format]);
            }
        }
    }

    let source_file = File::open(&flags.source)?;

    let writers: io::Result<Vec<BufWriter<File>>> = flags
        .output
        .iter()
        .map(|theme| -> io::Result<BufWriter<File>> {
            let mut path = flags.source.with_file_name(theme);
            if let Some(extension) = flags.source.extension() {
                path = path.with_extension(extension);
            }
            Ok(BufWriter::new(File::create(path)?))
        })
        .collect();
    let mut writers = writers?;

    let regex = regex::Regex::new(r"[a-fA-F0-9]{6}|rgba?\(.+\)|hsla?\(.+\)").unwrap();
    for line in BufReader::new(&source_file).lines() {
        let line = line?;

        for (theme, writer) in flags.output.iter().zip(&mut writers) {
            let mut copy = line.clone();

            for item in regex.find_iter(&line) {
                let lookup = match variant_from_color.get(item.as_str().to_lowercase().as_str()) {
                    Some(it) => it,
                    None => continue,
                };
                let label = color_from_variant.get(theme.as_str()).unwrap();
                let color_format = label.get(&lookup[1]).unwrap();
                let color_value = color_format.get(&lookup[2]).unwrap();

                copy.replace_range(item.range(), color_value);
            }
            writeln!(writer, "{}", copy)?;
        }
    }

    for mut writer in writers {
        writer.flush()?;
    }
    Ok(())
}

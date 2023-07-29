use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
    collections::HashMap,
};
use miniserde::json;

pub type Color = HashMap<String, String>;
pub type Palette = HashMap<String, Color>;


fn main() -> io::Result<()> {
    let mut color_from_variant: HashMap<&'static str, Palette> = HashMap::new();
    color_from_variant.insert("v1", json::from_str(include_str!("../palettes/v1/converted.json")).unwrap());
    color_from_variant.insert("latte", json::from_str(include_str!("../palettes/v2/latte.json")).unwrap());
    color_from_variant.insert("frappe", json::from_str(include_str!("../palettes/v2/frappe.json")).unwrap());
    color_from_variant.insert("macchiato", json::from_str(include_str!("../palettes/v2/macchiato.json")).unwrap());
    color_from_variant.insert("mocha", json::from_str(include_str!("../palettes/v2/mocha.json")).unwrap());

    let mut variant_from_color: HashMap<String, [String; 3]> = HashMap::new();
    for (variant, labels) in &color_from_variant {
        for (label, colors) in labels.iter() {
            for (format, value) in colors.iter() {
                variant_from_color.insert(value.to_owned(), [variant.to_string(), label.to_string(), format.to_string()]);
            }
        }
    }

    xflags::xflags! {
        /// Generate the other Catppuccin flavours off a template file written in one of them
        cmd puccinier {
            /// Set the source file to convert
            required source: PathBuf
            /// Set the themes to generate from the source file
            repeated -o, --output type: String
        }
    };

    let flags = Puccinier::from_env_or_exit();


    let source_file = File::open(&flags.source)?;

    let output_themes: Vec<String> = flags.output;

    let writers: io::Result<Vec<BufWriter<File>>> = output_themes
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


    let regex = regex::Regex::new(r"(?i)#([A-F0-9]{6}|[A-F0-9]{3})|rgba?\(.+\)|hsla?\(.+\)").unwrap();
    for line in BufReader::new(&source_file).lines() {
        let line = line?;

        for (theme, writer) in output_themes.iter().zip(&mut writers) {
            let mut copy = line.clone();

            regex.find_iter(&line)
                .filter_map(|item| {
                    let replacement: &str = item.as_str();
                    variant_from_color
                        .get(&replacement.to_lowercase())
                        .cloned()
                        .zip(Some(replacement))
                })
                .for_each(|(lookup, replacement)| {
                    let label: &Palette = color_from_variant.get(theme.as_str()).unwrap();
                    let color_format: &Color = label.get(&lookup[1]).unwrap();
                    let color_value: &String = color_format.get(&lookup[2]).unwrap();

                    copy = copy.replace(replacement, color_value);
                });

            writeln!(writer, "{}", copy)?;
        }
    }

    for mut writer in writers {
        writer.flush()?;
    }
    Ok(())
}

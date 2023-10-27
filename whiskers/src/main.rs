#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::cast_possible_truncation)]
use std::clone::Clone;

// we like truncating u32s into u8s around here
use clap::Parser;
use clap_stdin::FileOrStdin;
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};

use whiskers::frontmatter;
use whiskers::postprocess::postprocess;
use whiskers::template::{self, helpers};

#[derive(clap::ValueEnum, Clone, Debug)]
enum Flavor {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl From<Flavor> for catppuccin::Flavour {
    fn from(value: Flavor) -> Self {
        match value {
            Flavor::Latte => Self::Latte,
            Flavor::Frappe => Self::Frappe,
            Flavor::Macchiato => Self::Macchiato,
            Flavor::Mocha => Self::Mocha,
        }
    }
}

#[derive(Clone, Debug)]
struct Override {
    pub key: String,
    pub value: serde_json::Value,
}

fn parse_override(s: &str) -> Result<Override> {
    let kvpair = s.split_once('=');
    if let Some((key, value)) = kvpair {
        return Ok(Override {
            key: key.trim().to_string(),
            value: serde_json::Value::String(value.trim().to_owned()),
        });
    }
    Err(eyre!("invalid override, expected 'key=value', got '{}'", s))
}

#[derive(clap::Parser, Debug)]
struct Args {
    /// Path to the template file to render, or `-` for stdin
    #[arg(required_unless_present = "list_helpers")]
    template: Option<FileOrStdin>,

    /// Flavor to get colors from
    #[arg(value_enum, required_unless_present = "list_helpers")]
    flavor: Option<Flavor>,

    /// The overrides to apply to the template in key=value format
    #[arg(long("override"), value_parser(parse_override))]
    overrides: Vec<Override>,

    /// List all template helpers in markdown format
    #[arg(short, long)]
    list_helpers: bool,
}

fn contextualize_overrides(overrides: Vec<Override>, ctx: &serde_json::Value) -> Vec<Override> {
    let map = ctx.as_object().expect("base context is an object value");
    overrides
        .into_iter()
        .map(|o| {
            let lookup = o.value.as_str().expect("override values are strings");
            let value = map.get(lookup).map(Clone::clone).unwrap_or(o.value);
            Override { key: o.key, value }
        })
        .collect()
}

fn overrides_to_map(overrides: Vec<Override>) -> serde_json::Map<String, serde_json::Value> {
    overrides.into_iter().map(|o| (o.key, o.value)).collect()
}

fn merge_contexts(
    ctx: serde_json::Value,
    frontmatter: Option<serde_json::Value>,
    overrides: Vec<Override>,
) -> serde_json::Value {
    type Map = serde_json::Map<String, serde_json::Value>;
    let mut merged = Map::new();

    let overrides = contextualize_overrides(overrides, &ctx);

    merged.extend(
        serde_json::from_value::<Map>(ctx).expect("base context is deserializable into a map"),
    );

    if let Some(frontmatter) = frontmatter {
        merged.extend(
            serde_json::from_value::<Map>(frontmatter)
                .expect("frontmatter is deserializable into a map"),
        );
    }

    merged.extend(overrides_to_map(overrides));

    serde_json::Value::Object(merged)
}

fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .panic_section("Consider reporting this issue: https://github.com/catppuccin/toolbox")
        .display_env_section(false)
        .install()?;

    let args = Args::parse();

    if args.list_helpers {
        list_helpers();
        return Ok(());
    }

    let template = &args
        .template
        .expect("template_path is guaranteed to be set");

    let flavor = args.flavor.expect("flavor is guaranteed to be set");

    let reg = template::make_registry();

    let ctx = template::make_context(flavor.into());
    let (content, frontmatter) = frontmatter::render_and_parse(template, &reg, &ctx);

    let ctx = merge_contexts(ctx, frontmatter, args.overrides);

    let result = reg
        .render_template(content, &ctx)
        .wrap_err("Failed to render template")?;
    let result = postprocess(&result);
    println!("{result}");

    Ok(())
}

fn list_helpers() {
    for helper in helpers() {
        print!("- `{}", helper.name);
        for arg in helper.args {
            print!(" {arg}");
        }
        println!("` : {}", helper.description);
        for (before, after) in helper.examples {
            println!("    - `{{{{ {} {} }}}}` → {}", helper.name, before, after);
        }
    }
}


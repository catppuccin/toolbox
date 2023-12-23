#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
// we like truncating u32s into u8s around here
#![allow(clippy::cast_possible_truncation)]

use std::{
    env, fmt, fs,
    io::Write,
    path::{Path, PathBuf},
    process,
};

use clap::Parser;
use clap_stdin::FileOrStdin;
use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use json_patch::merge;
use serde_json::{json, Value};

use catppuccin_whiskers::{
    frontmatter,
    postprocess::postprocess,
    template::{self, helpers},
    Map,
};

#[derive(clap::ValueEnum, Clone, Debug)]
enum Flavor {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
    All,
}

#[allow(clippy::fallible_impl_from)]
impl From<Flavor> for catppuccin::Flavour {
    fn from(value: Flavor) -> Self {
        match value {
            Flavor::Latte => Self::Latte,
            Flavor::Frappe => Self::Frappe,
            Flavor::Macchiato => Self::Macchiato,
            Flavor::Mocha => Self::Mocha,
            // This should never be called, but it's here to satisfy the compiler.
            Flavor::All => panic!(),
        }
    }
}

impl fmt::Display for Flavor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Latte => write!(f, "latte"),
            Self::Frappe => write!(f, "frappe"),
            Self::Macchiato => write!(f, "macchiato"),
            Self::Mocha => write!(f, "mocha"),
            Self::All => write!(f, "all"),
        }
    }
}

fn parse_overrides(s: &str) -> Result<Value> {
    match serde_json::from_str(s) {
        Ok(json) => Ok(json),
        Err(err) => Err(eyre!("invalid JSON syntax in overrides: {}", err)),
    }
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the template file to render, or `-` for stdin
    #[arg(required_unless_present = "list_helpers")]
    template: Option<FileOrStdin>,

    /// Flavor to get colors from
    #[arg(value_enum, required_unless_present = "list_helpers")]
    flavor: Option<Flavor>,

    /// The overrides to apply to the template in JSON/YAML format
    #[arg(long, value_parser(parse_overrides))]
    overrides: Option<Value>,

    /// Path to write to instead of stdout
    #[arg(short, long)]
    output_path: Option<PathBuf>,

    /// Instead of printing a result, check if anything would change
    #[arg(long)]
    check: Option<PathBuf>,

    /// List all template helpers in Markdown format
    #[arg(short, long)]
    list_helpers: bool,
}

fn merge_contexts_all(ctx: &Value, frontmatter: &Map) -> Value {
    let ctx = ctx.as_object().expect("ctx is an object").clone();

    // Slight inefficiency here as the root context variables are
    // also duplicated into each flavor.
    let flavors: Map = ctx
        .into_iter()
        .map(|(name, ctx)| {
            let flavor = frontmatter
                .get(&name)
                .expect("flavor exists in frontmatter");
            let merged_ctx = merge_contexts(ctx, flavor);
            (name, merged_ctx)
        })
        .collect();

    let merged = json!({ "flavors": flavors });

    // QUESTION:
    // Do we want to allow root context variables when running in single file mode?
    // E.g.

    // ---
    // title: "Catppuccin For <Port>" <--- This currently gets duplicated for each flavor --->
    // ---
    //
    // {{title}} <--- Do we want to allow this even though it's outside of the {{#each}} loop --->
    //
    // {{each ...}}
    // ...
    // {{/each}}

    // It's quite annoying to differentiate as it would involve using something like
    // regex to detect if a variable contains "{{ }}" and hold that in a separate
    // object to extend the "merged" map with here, but it would mean that we
    // could define variables that can be used outside the "each" loop.

    // if let Some(root) = root_frontmatter {
    //     let obj = merged.as_object_mut().expect("flavors is an object");
    //     obj.extend(root);
    //     obj.extend(overrides_to_map(overrides.into()));
    // }

    serde_json::to_value(merged).expect("merged context is serializable")
}

fn merge_contexts(ctx: Value, frontmatter: &Value) -> Value {
    let mut merged = ctx;
    if !frontmatter.is_null() {
        merge(&mut merged, frontmatter);
    }
    merged
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
        .expect("template_path is guaranteed to be set")
        .contents()
        .expect("template contents are readable");

    let flavor = args.flavor.expect("flavor is guaranteed to be set");
    let flavor_string = flavor.to_string();

    let reg = template::make_registry();

    let (content, ctx) = if matches!(flavor, Flavor::All) {
        let ctx = template::make_context_all();
        let (content, frontmatter) =
            frontmatter::render_and_parse_all(template, &args.overrides, &reg, &ctx);
        let merged_ctx = merge_contexts_all(&ctx, &frontmatter);
        (content, merged_ctx)
    } else {
        let ctx = template::make_context(&flavor.into());
        let (content, frontmatter) = frontmatter::render_and_parse(
            template,
            args.overrides,
            flavor_string.as_str(),
            &reg,
            &ctx,
        );
        let merged_ctx = merge_contexts(ctx, &frontmatter);
        (content, merged_ctx)
    };

    let result = reg
        .render_template(content, &ctx)
        .wrap_err("Failed to render template")?;
    let result = postprocess(&result);

    if let Some(expected_path) = args.check {
        let expected = fs::read_to_string(&expected_path)?;
        if result != expected {
            eprintln!("Templating would result in changes.");
            invoke_difftool(&result, &expected_path)?;
            process::exit(1);
        }
    } else if let Some(output_path) = args.output_path {
        fs::write(output_path, result)?;
    } else {
        print!("{result}");
    }

    Ok(())
}

fn invoke_difftool(actual: &str, expected_path: &Path) -> Result<(), color_eyre::eyre::Error> {
    let tool = env::var("DIFFTOOL").unwrap_or_else(|_| "diff".to_string());

    let mut actual_file = tempfile::NamedTempFile::new()?;
    write!(&mut actual_file, "{actual}")?;
    if let Ok(mut child) = process::Command::new(tool)
        .args([actual_file.path(), &expected_path])
        .spawn()
    {
        child.wait()?;
    } else {
        eprintln!("warning: Can't display diff, try setting $DIFFTOOL.");
    }

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

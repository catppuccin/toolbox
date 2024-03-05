use handlebars::Handlebars;
use handlebars::HelperDef;
use indexmap::IndexMap;
use serde_json::Value;

use crate::helper;

pub struct Helper {
    pub name: &'static str,
    pub description: &'static str,
    pub args: &'static [&'static str],
    pub examples: &'static [(&'static str, &'static str)],
    handler: Box<dyn HelperDef + Send + Sync>,
}

#[allow(clippy::too_many_lines)]
pub fn helpers() -> Vec<Helper> {
    vec![
        Helper {
            name: "uppercase",
            description: "Convert a string to uppercase.",
            args: &["string"],
            examples: &[("\"hello\"", "`HELLO`")],
            handler: Box::new(helper::uppercase),
        },
        Helper {
            name: "lowercase",
            description: "Convert a string to lowercase.",
            args: &["string"],
            examples: &[("\"HELLO\"", "`hello`")],
            handler: Box::new(helper::lowercase),
        },
        Helper {
            name: "titlecase",
            description: "Convert a string to titlecase.",
            args: &["string"],
            examples: &[("\"hello there\"", "`Hello There`")],
            handler: Box::new(helper::titlecase),
        },
        Helper {
            name: "trunc",
            description: "Format a number to a string with a given number of places.",
            args: &["number", "places"],
            examples: &[("3.14159265 2", "`3.14`")],
            handler: Box::new(helper::trunc),
        },
        Helper {
            name: "lighten",
            description: "Lighten a color by a percentage.",
            args: &["color", "amount"],
            examples: &[("red 0.1", "`f8bacc` / `hsl(343, 81%, 85%)`")],
            handler: Box::new(helper::lighten),
        },
        Helper {
            name: "darken",
            description: "Darken a color by a percentage.",
            args: &["color", "amount"],
            examples: &[("red 0.1", "`ee5c85` / `hsl(343, 81%, 65%)`")],
            handler: Box::new(helper::darken),
        },
        Helper {
            name: "mix",
            description: "Mix two colors together in a given ratio.",
            args: &["color_a", "color_b", "ratio"],
            examples: &[("red base 0.3", "`5e4054` (30% red, 70% base)")],
            handler: Box::new(helper::mix),
        },
        Helper {
            name: "opacity",
            description: "Set the opacity of a color.",
            args: &["color", "amount"],
            examples: &[("red 0.5", "`hsla(343, 81%, 75%, 0.50)`")],
            handler: Box::new(helper::opacity),
        },
        Helper {
            name: "unquote",
            description: "Marks a value to be unquoted. Mostly useful for maintaining JSON syntax highlighting in template files when a non-string value is needed.",
            args: &["value"],
            examples: &[("isLight true", "`true` (the surrounding quotation marks have been removed)")],
            handler: Box::new(helper::unquote),
        },
        Helper {
            name: "rgb",
            description: "Convert a color to CSS RGB format.",
            args: &["color"],
            examples: &[("red", "`rgb(243, 139, 168)`")],
            handler: Box::new(helper::rgb),
        },
        Helper {
            name: "rgba",
            description: "Convert a color to CSS RGBA format.",
            args: &["color"],
            examples: &[("(opacity red 0.6)", "`rgba(243, 139, 168, 0.60)`")],
            handler: Box::new(helper::rgba),
        },
        Helper {
            name: "hsl",
            description: "Convert a color to CSS HSL format.",
            args: &["color"],
            examples: &[("red", "`hsl(343, 81%, 75%)`")],
            handler: Box::new(helper::hsl),
        },
        Helper {
            name: "hsla",
            description: "Convert a color to CSS HSLA format.",
            args: &["color"],
            examples: &[("(opacity red 0.6)", "`hsla(343, 81%, 75%, 0.60)`")],
            handler: Box::new(helper::hsla),
        },
        Helper {
            name: "red_i",
            description: "Get the red channel of a color as an integer from 0 to 255.",
            args: &["color"],
            examples: &[("red", "`243`")],
            handler: Box::new(helper::red_i),
        },
        Helper {
            name: "green_i",
            description: "Get the green channel of a color as an integer from 0 to 255.",
            args: &["color"],
            examples: &[("red", "`139`")],
            handler: Box::new(helper::green_i),
        },
        Helper {
            name: "blue_i",
            description: "Get the blue channel of a color as an integer from 0 to 255.",
            args: &["color"],
            examples: &[("red", "`168`")],
            handler: Box::new(helper::blue_i),
        },
        Helper {
            name: "alpha_i",
            description: "Get the alpha channel of a color as an integer from 0 to 255.",
            args: &["color"],
            examples: &[("(opacity red 0.6)", "`153`")],
            handler: Box::new(helper::alpha_i),
        },
        Helper {
            name: "red_f",
            description: "Get the red channel of a color as a float from 0 to 1.",
            args: &["color"],
            examples: &[("red", "`0.95` (truncated to 2 places)")],
            handler: Box::new(helper::red_f),
        },
        Helper {
            name: "green_f",
            description: "Get the green channel of a color as a float from 0 to 1.",
            args: &["color"],
            examples: &[("red", "`0.55` (truncated to 2 places)")],
            handler: Box::new(helper::green_f),
        },
        Helper {
            name: "blue_f",
            description: "Get the blue channel of a color as a float from 0 to 1.",
            args: &["color"],
            examples: &[("red", "`0.66` (truncated to 2 places)")],
            handler: Box::new(helper::blue_f),
        },
        Helper {
            name: "alpha_f",
            description: "Get the alpha channel of a color as a float from 0 to 1.",
            args: &["color"],
            examples: &[("(opacity red 0.6)", "`0.60` (truncated to 2 places)")],
            handler: Box::new(helper::alpha_f),
        },
        Helper {
            name: "red_h",
            description: "Get the red channel of a color as hex digits from 00 to FF.",
            args: &["color"],
            examples: &[("red", "`f3`")],
            handler: Box::new(helper::red_h),
        },
        Helper {
            name: "green_h",
            description: "Get the green channel of a color as hex digits from 00 to FF.",
            args: &["color"],
            examples: &[("red", "`8b`")],
            handler: Box::new(helper::green_h),
        },
        Helper {
            name: "blue_h",
            description: "Get the blue channel of a color as hex digits from 00 to FF.",
            args: &["color"],
            examples: &[("red", "`a8`")],
            handler: Box::new(helper::blue_h),
        },
        Helper {
            name: "alpha_h",
            description: "Get the alpha channel of a color as hex digits from 00 to FF.",
            args: &["color"],
            examples: &[("(opacity red 0.6)", "`99`")],
            handler: Box::new(helper::alpha_h),
        },
        Helper {
            name: "darklight",
            description: "Choose a value depending on the current flavor. Latte is light, while Frappé, Macchiato, and Mocha are all dark.",
            args: &["if-dark", "if-light"],
            examples: &[("\"Night\" \"Day\"", "`Day` on Latte, `Night` on other flavors")],
            handler: Box::new(helper::darklight),
        },
    ]
}

#[must_use]
pub fn make_registry() -> Handlebars<'static> {
    let mut reg = Handlebars::new();
    for helper in helpers() {
        reg.register_helper(helper.name, helper.handler);
    }
    reg.set_strict_mode(true);
    reg
}

#[must_use]
#[allow(clippy::missing_panics_doc)] // panic here implies an internal issue
pub fn make_context_all() -> Value {
    let ctx: IndexMap<String, Value> = catppuccin::PALETTE
        .into_iter()
        .map(|f| (f.name.identifier().to_string(), make_context(f)))
        .collect();
    serde_json::to_value(ctx).expect("context is serializable into json")
}

#[must_use]
#[allow(clippy::missing_panics_doc)] // panic here implies an internal issue
pub fn make_context(flavor: &catppuccin::Flavor) -> Value {
    let color_map: IndexMap<String, String> = flavor
        .colors
        .into_iter()
        .map(|c| {
            (
                c.name.identifier().to_string(),
                format!("{:02x}{:02x}{:02x}", c.rgb.r, c.rgb.g, c.rgb.b),
            )
        })
        .collect();

    let mut context =
        serde_json::to_value(color_map.clone()).expect("color names & hexcodes can be serialized");

    context["flavor"] = flavor.name.identifier().to_string().into();
    context["flavorName"] = flavor.name.to_string().into();
    context["isLight"] = (!flavor.dark).into();
    context["isDark"] = flavor.dark.into();
    context["colors"] =
        serde_json::to_value(color_map).expect("color names & hexcodes can be serialized");

    context
}

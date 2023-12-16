use handlebars::Handlebars;
use handlebars::HelperDef;
use indexmap::IndexMap;

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

fn flavor_priority(flavor: &str) -> u32 {
    match flavor {
        "latte" => 1,
        "frappe" => 2,
        "macchiato" => 3,
        "mocha" => 4,
        _ => unreachable!(),
    }
}

fn color_priority(color: &str) -> u32 {
    match color {
        "rosewater" => 1,
        "flamingo" => 2,
        "pink" => 3,
        "mauve" => 4,
        "red" => 5,
        "maroon" => 6,
        "peach" => 7,
        "yellow" => 8,
        "green" => 9,
        "teal" => 10,
        "sky" => 11,
        "sapphire" => 12,
        "blue" => 13,
        "lavender" => 14,
        "text" => 15,
        "subtext1" => 16,
        "subtext0" => 17,
        "overlay2" => 18,
        "overlay1" => 19,
        "overlay0" => 20,
        "surface2" => 21,
        "surface1" => 22,
        "surface0" => 23,
        "base" => 24,
        "mantle" => 25,
        "crust" => 26,
        _ => unreachable!(),
    }
}

#[must_use]
#[allow(clippy::missing_panics_doc)] // panic here implies an internal issue
pub fn make_context_all() -> serde_json::Value {
    let mut flavours: IndexMap<&str, serde_json::Value> = catppuccin::Flavour::into_iter()
        .map(|f| (f.name(), make_context(f)))
        .collect();
    flavours.sort_by(|a, _, b, _| flavor_priority(a).cmp(&flavor_priority(b)));

    let context = IndexMap::from([("flavors", flavours)]);

    serde_json::to_value(context).expect("flavours can be serialized")
}

#[must_use]
#[allow(clippy::missing_panics_doc)] // panic here implies an internal issue
pub fn make_context(flavor: catppuccin::Flavour) -> serde_json::Value {
    let colors = flavor.colours();

    let mut color_map: IndexMap<String, String> = colors
        .into_fields_iter()
        .map(|(name, c)| (name.to_string(), c.hex().to_ascii_lowercase()))
        .collect();
    color_map.sort_by(|a, _, b, _| color_priority(a).cmp(&color_priority(b)));

    let mut context =
        serde_json::to_value(color_map.clone()).expect("color names & hexcodes can be serialized");

    context["flavor"] = flavor.name().into();
    context["isLight"] = (flavor == catppuccin::Flavour::Latte).into();
    context["isDark"] = (flavor != catppuccin::Flavour::Latte).into();
    context["colors"] =
        serde_json::to_value(color_map).expect("color names & hexcodes can be serialized");

    context
}

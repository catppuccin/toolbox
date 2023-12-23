#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
// we like truncating u32s into u8s around here
#![allow(clippy::cast_possible_truncation)]

use serde_json::Value;

pub mod frontmatter;
mod helper;
mod parse;
pub mod postprocess;
pub mod template;

pub type Map = serde_json::Map<String, Value>;

const COLOR_NAMES: [&str; 26] = [
    "rosewater",
    "flamingo",
    "pink",
    "mauve",
    "red",
    "maroon",
    "peach",
    "yellow",
    "green",
    "teal",
    "sky",
    "sapphire",
    "blue",
    "lavender",
    "text",
    "subtext1",
    "subtext0",
    "overlay2",
    "overlay1",
    "overlay0",
    "surface2",
    "surface1",
    "surface0",
    "base",
    "mantle",
    "crust",
];

const FLAVOR_NAMES: [&str; 4] = ["latte", "frappe", "macchiato", "mocha"];

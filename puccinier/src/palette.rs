use std::collections::HashMap;

pub type Color = HashMap<String, String>;
pub type Palette = HashMap<String, Color>;

lazy_static::lazy_static! {
    pub static ref COLOR_FROM_VARIANT: HashMap<String, Palette> = {
        let mut catppuccin = HashMap::new();
        catppuccin.insert(String::from("v1"), serde_json::from_str(include_str!("../palettes/v1/converted.json")).unwrap());
        catppuccin.insert(String::from("latte"), serde_json::from_str(include_str!("../palettes/v2/latte.json")).unwrap());
        catppuccin.insert(String::from("frappe"), serde_json::from_str(include_str!("../palettes/v2/frappe.json")).unwrap());
        catppuccin.insert(String::from("macchiato"), serde_json::from_str(include_str!("../palettes/v2/macchiato.json")).unwrap());
        catppuccin.insert(String::from("mocha"), serde_json::from_str(include_str!("../palettes/v2/mocha.json")).unwrap());

        catppuccin
    };

    pub static ref VARIANT_FROM_COLOR: HashMap<String, [&'static String; 3]> = {
        let mut lookup = HashMap::new();

        for (variant, labels) in &*COLOR_FROM_VARIANT {
            for (label, colors) in labels.iter() {
                for (format, value) in colors.iter() {
                    lookup.insert(value.to_owned(), [variant, label, format]);
                }
            }
        }

        lookup
    };
}

use anyhow::Context;
use css_colors::Color as _;
use indexmap::IndexMap;
use std::collections::HashMap;
use crate::cli::ColorOverrides;

// a frankenstein mix of Catppuccin & css_colors types to get all the
// functionality we want.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Palette {
    pub flavors: IndexMap<String, Flavor>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Flavor {
    pub name: String,
    pub identifier: String,
    pub dark: bool,
    pub light: bool,
    pub colors: IndexMap<String, Color>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Color {
    pub name: String,
    pub identifier: String,
    pub accent: bool,
    pub hex: String,
    pub hex_format: String,
    pub rgb: RGB,
    pub hsl: HSL,
    pub opacity: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct HSL {
    pub h: u16,
    pub s: f32,
    pub l: f32,
}
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to parse hex color: {0}")]
    ParseHex(#[from] std::num::ParseIntError),
}

/// attempt to canonicalize a hex string, and convert it to the provided format
fn format_hex(hex: &str, hex_format: String) -> Result<String, anyhow::Error> {
    let hex = hex.trim_start_matches('#');
    let alpha = if hex.len() == 8 {&hex[6..8]} else {""};
    let hex = [("r", &hex[0..2]), ("g", &hex[2..4]), ("b", &hex[4..6]), ("a", alpha)].map(|(k,v)| (k.to_string(), v.to_string()));

    let mut hex_ctx = HashMap::from(hex.clone());
    hex_ctx.extend(hex.map(|(k,v)| (k.to_uppercase(), v.to_uppercase())));

    let hex_ctx = tera::Context::from_serialize(&hex_ctx).context("Hex template render failed")?;

    tera::Tera::one_off(hex_format.as_str(), &hex_ctx, false).context("Hex template render failed")
}

fn color_from_hex(
    hex: &str,
    blueprint: &catppuccin::Color,
    hex_format: String,
) -> Result<Color, Error> {
    let i = u32::from_str_radix(hex, 16)?;
    let rgb = RGB {
        r: ((i >> 16) & 0xff) as u8,
        g: ((i >> 8) & 0xff) as u8,
        b: (i & 0xff) as u8,
    };
    let hsl = css_colors::rgb(rgb.r, rgb.g, rgb.b).to_hsl();
    let hex = format_hex(hex, hex_format.clone()).context("Hex formatting failed").unwrap();
    Ok(Color {
        name: blueprint.name.to_string(),
        identifier: blueprint.name.identifier().to_string(),
        accent: blueprint.accent,
        hex,
        hex_format,
        rgb,
        hsl: HSL {
            h: hsl.h.degrees(),
            s: hsl.s.as_f32(),
            l: hsl.l.as_f32(),
        },
        opacity: 255,
    })
}

fn color_from_catppuccin(color: &catppuccin::Color, hex_format: String) -> Color {
    let hex = format_hex(&color.hex.to_string(), hex_format.clone()).context("Hex formatting failed").unwrap();
    Color {
        name: color.name.to_string(),
        identifier: color.name.identifier().to_string(),
        accent: color.accent,
        hex,
        hex_format,
        rgb: RGB {
            r: color.rgb.r,
            g: color.rgb.g,
            b: color.rgb.b,
        },
        hsl: HSL {
            h: color.hsl.h.round() as u16,
            s: color.hsl.s as f32,
            l: color.hsl.l as f32,
        },
        opacity: 255,
    }
}

/// Build a [`Palette`] from [`catppuccin::PALETTE`], optionally applying color overrides.
pub fn build_palette(
    hex_format: String,
    color_overrides: Option<&ColorOverrides>,
) -> Result<Palette, Error> {
    // make a `Color` from a `catppuccin::Color`, taking into account `color_overrides`.
    // overrides apply in this order:
    // 1. base color
    // 2. "all" override
    // 3. flavor override
    let make_color =
        |color: &catppuccin::Color, flavor_name: catppuccin::FlavorName| -> Result<Color, Error> {
            let flavor_override = color_overrides
                .map(|co| match flavor_name {
                    catppuccin::FlavorName::Latte => &co.latte,
                    catppuccin::FlavorName::Frappe => &co.frappe,
                    catppuccin::FlavorName::Macchiato => &co.macchiato,
                    catppuccin::FlavorName::Mocha => &co.mocha,
                })
                .and_then(|o| o.get(color.name.identifier()).cloned())
                .map(|s| color_from_hex(&s, color, hex_format.clone()))
                .transpose()?;

            let all_override = color_overrides
                .and_then(|co| co.all.get(color.name.identifier()).cloned())
                .map(|s| color_from_hex(&s, color, hex_format.clone()))
                .transpose()?;

            Ok(flavor_override
                .or(all_override)
                .unwrap_or_else(|| color_from_catppuccin(color, hex_format.clone())))
        };

    let mut flavors = IndexMap::new();
    for flavor in &catppuccin::PALETTE {
        let mut colors = IndexMap::new();
        for color in flavor {
            colors.insert(
                color.name.identifier().to_string(),
                make_color(color, flavor.name)?,
            );
        }
        flavors.insert(
            flavor.name.identifier().to_string(),
            Flavor {
                name: flavor.name.to_string(),
                identifier: flavor.name.identifier().to_string(),
                dark: flavor.dark,
                light: !flavor.dark,
                colors,
            },
        );
    }
    Ok(Palette { flavors })
}

impl Palette {
    #[must_use]
    pub fn iter(&self) -> indexmap::map::Iter<String, Flavor> {
        self.flavors.iter()
    }
}

impl<'a> IntoIterator for &'a Palette {
    type Item = (&'a String, &'a Flavor);
    type IntoIter = indexmap::map::Iter<'a, String, Flavor>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Flavor {
    #[must_use]
    pub fn iter(&self) -> indexmap::map::Iter<String, Color> {
        self.colors.iter()
    }
}

impl<'a> IntoIterator for &'a Flavor {
    type Item = (&'a String, &'a Color);
    type IntoIter = indexmap::map::Iter<'a, String, Color>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

fn rgb_to_hex(rgb: &RGB, opacity: u8, hex_format: String) -> String {
    let hex = format!("{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b);
    let hexa = format!("{hex}{opacity:02x}");
    format_hex(if opacity < 255 {&hexa} else {&hex}, hex_format).context("Hex formatting failed").unwrap()
}

impl Color {
    fn from_hsla(hsla: css_colors::HSLA, blueprint: &Self, hex_format: String) -> Self {
        let rgb = hsla.to_rgb();
        let rgb = RGB {
            r: rgb.r.as_u8(),
            g: rgb.g.as_u8(),
            b: rgb.b.as_u8(),
        };
        let hsl = HSL {
            h: hsla.h.degrees(),
            s: hsla.s.as_f32(),
            l: hsla.l.as_f32(),
        };
        let opacity = hsla.a.as_u8();
        Self {
            name: blueprint.name.clone(),
            identifier: blueprint.identifier.clone(),
            accent: blueprint.accent,
            hex: rgb_to_hex(&rgb, opacity, hex_format.clone()),
            hex_format,
            rgb,
            hsl,
            opacity,
        }
    }

    fn from_rgba(rgba: css_colors::RGBA, blueprint: &Self, hex_format: String) -> Self {
        let hsl = rgba.to_hsl();
        let rgb = RGB {
            r: rgba.r.as_u8(),
            g: rgba.g.as_u8(),
            b: rgba.b.as_u8(),
        };
        let hsl = HSL {
            h: hsl.h.degrees(),
            s: hsl.s.as_f32(),
            l: hsl.l.as_f32(),
        };
        let opacity = rgba.a.as_u8();
        Self {
            name: blueprint.name.clone(),
            identifier: blueprint.identifier.clone(),
            accent: blueprint.accent,
            hex: rgb_to_hex(&rgb, opacity, hex_format.clone()),
            hex_format,
            rgb,
            hsl,
            opacity,
        }
    }

    #[must_use]
    pub fn mix(base: &Self, blend: &Self, amount: f64, hex_format: String) -> Self {
        let amount = (amount * 100.0).clamp(0.0, 100.0).round() as u8;
        let blueprint = base;
        let base: css_colors::RGBA = base.into();
        let base = base.to_rgba();
        let blend: css_colors::RGBA = blend.into();
        let result = base.mix(blend, css_colors::percent(amount));
        Self::from_rgba(result, blueprint, hex_format)
    }

    #[must_use]
    pub fn mod_hue(&self, hue: i32, hex_format: String) -> Self {
        let mut hsl: css_colors::HSL = self.into();
        hsl.h = css_colors::deg(hue);
        Self::from_hsla(hsl.to_hsla(), self, hex_format)
    }

    #[must_use]
    pub fn add_hue(&self, hue: i32, hex_format: String) -> Self {
        let hsl: css_colors::HSL = self.into();
        let hsl = hsl.spin(css_colors::deg(hue));
        Self::from_hsla(hsl.to_hsla(), self, hex_format)
    }

    #[must_use]
    pub fn sub_hue(&self, hue: i32, hex_format: String) -> Self {
        let hsl: css_colors::HSL = self.into();
        let hsl = hsl.spin(-css_colors::deg(hue));
        Self::from_hsla(hsl.to_hsla(), self, hex_format)
    }

    #[must_use]
    pub fn mod_saturation(&self, saturation: u8, hex_format: String) -> Self {
        let mut hsl: css_colors::HSL = self.into();
        hsl.s = css_colors::percent(saturation);
        Self::from_hsla(hsl.to_hsla(), self, hex_format)
    }

    #[must_use]
    pub fn add_saturation(&self, saturation: u8, hex_format: String) -> Self {
        let hsl: css_colors::HSL = self.into();
        let hsl = hsl.saturate(css_colors::percent(saturation));
        Self::from_hsla(hsl.to_hsla(), self, hex_format)
    }

    #[must_use]
    pub fn sub_saturation(&self, saturation: u8, hex_format: String) -> Self {
        let hsl: css_colors::HSL = self.into();
        let hsl = hsl.desaturate(css_colors::percent(saturation));
        Self::from_hsla(hsl.to_hsla(), self, hex_format)
    }

    #[must_use]
    pub fn mod_lightness(&self, lightness: u8, hex_format: String) -> Self {
        let mut hsl: css_colors::HSL = self.into();
        hsl.l = css_colors::percent(lightness);
        Self::from_hsla(hsl.to_hsla(), self, hex_format)
    }

    #[must_use]
    pub fn add_lightness(&self, lightness: u8, hex_format: String) -> Self {
        let hsl: css_colors::HSL = self.into();
        let hsl = hsl.lighten(css_colors::percent(lightness));
        Self::from_hsla(hsl.to_hsla(), self, hex_format)
    }

    #[must_use]
    pub fn sub_lightness(&self, lightness: u8, hex_format: String) -> Self {
        let hsl: css_colors::HSL = self.into();
        let hsl = hsl.darken(css_colors::percent(lightness));
        Self::from_hsla(hsl.to_hsla(), self, hex_format)
    }

    #[must_use]
    pub fn mod_opacity(&self, opacity: f32, hex_format: String) -> Self {
        let opacity = (opacity * 255.0).round() as u8;
        Self {
            opacity,
            hex: rgb_to_hex(&self.rgb, opacity, hex_format),
            ..self.clone()
        }
    }

    #[must_use]
    pub fn add_opacity(&self, opacity: f32, hex_format: String) -> Self {
        let opacity = (opacity * 255.0).round() as u8;
        let opacity = self.opacity.saturating_add(opacity);
        Self {
            opacity,
            hex: rgb_to_hex(&self.rgb, opacity, hex_format),
            ..self.clone()
        }
    }

    #[must_use]
    pub fn sub_opacity(&self, opacity: f32, hex_format: String) -> Self {
        let opacity = (opacity * 255.0).round() as u8;
        let opacity = self.opacity.saturating_sub(opacity);
        Self {
            opacity,
            hex: rgb_to_hex(&self.rgb, opacity, hex_format),
            ..self.clone()
        }
    }
}

impl From<&Color> for css_colors::RGB {
    fn from(c: &Color) -> Self {
        Self {
            r: css_colors::Ratio::from_u8(c.rgb.r),
            g: css_colors::Ratio::from_u8(c.rgb.g),
            b: css_colors::Ratio::from_u8(c.rgb.b),
        }
    }
}

impl From<&Color> for css_colors::RGBA {
    fn from(c: &Color) -> Self {
        Self {
            r: css_colors::Ratio::from_u8(c.rgb.r),
            g: css_colors::Ratio::from_u8(c.rgb.g),
            b: css_colors::Ratio::from_u8(c.rgb.b),
            a: css_colors::Ratio::from_u8(c.opacity),
        }
    }
}

impl From<&Color> for css_colors::HSL {
    fn from(c: &Color) -> Self {
        Self {
            h: css_colors::Angle::new(c.hsl.h),
            s: css_colors::Ratio::from_f32(c.hsl.s),
            l: css_colors::Ratio::from_f32(c.hsl.l),
        }
    }
}

impl From<&Color> for css_colors::HSLA {
    fn from(c: &Color) -> Self {
        Self {
            h: css_colors::Angle::new(c.hsl.h),
            s: css_colors::Ratio::from_f32(c.hsl.s),
            l: css_colors::Ratio::from_f32(c.hsl.l),
            a: css_colors::Ratio::from_u8(c.opacity),
        }
    }
}

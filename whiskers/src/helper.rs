use base64::Engine;
use css_colors::{Color, Ratio, HSLA, RGBA};
use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, HelperResult, Output, RenderContext,
    RenderError,
};

use ::titlecase::titlecase as titlecase_ext;
use serde_json::Value;

use crate::parse::ColorExt;

impl From<crate::parse::Error> for RenderError {
    fn from(value: crate::parse::Error) -> Self {
        Self::from_error("Failed to parse hex string", value)
    }
}

handlebars_helper!(uppercase: |s: String| s.to_uppercase());
handlebars_helper!(lowercase: |s: String| s.to_lowercase());
handlebars_helper!(titlecase: |s: String| titlecase_ext(&s));
handlebars_helper!(trunc: |number: f32, places: usize| format!("{number:.places$}"));
handlebars_helper!(lighten: |color: String, weight: f32| {
    HSLA::from_hex(&color)?.lighten(Ratio::from_f32(weight)).to_hex()
});
handlebars_helper!(darken: |color: String, weight: f32| {
    HSLA::from_hex(&color)?.darken(Ratio::from_f32(weight)).to_hex()
});
handlebars_helper!(mix: |color_a: String, color_b: String, t: f32| {
    HSLA::from_hex(&color_a)?.mix(HSLA::from_hex(&color_b)?, Ratio::from_f32(t)).to_hex()
});
handlebars_helper!(opacity: |color: String, amount: f32| {
    HSLA::from_hex(&color)?.fade(Ratio::from_f32(amount)).to_hex()
});
handlebars_helper!(rgb: |color: String| {
    RGBA::from_hex(&color)?.to_rgb().to_string()
});
handlebars_helper!(rgba: |color: String| {
    RGBA::from_hex(&color)?.to_string()
});
handlebars_helper!(hsl: |color: String| {
    HSLA::from_hex(&color)?.to_hsl().to_string()
});
handlebars_helper!(hsla: |color: String| {
    HSLA::from_hex(&color)?.to_string()
});
handlebars_helper!(red_i: |color: String| {
    RGBA::from_hex(&color)?.r.as_u8()
});
handlebars_helper!(green_i: |color: String| {
    RGBA::from_hex(&color)?.g.as_u8()
});
handlebars_helper!(blue_i: |color: String| {
    RGBA::from_hex(&color)?.b.as_u8()
});
handlebars_helper!(alpha_i: |color: String| {
    RGBA::from_hex(&color)?.a.as_u8()
});
handlebars_helper!(red_f: |color: String| {
    RGBA::from_hex(&color)?.r.as_f32()
});
handlebars_helper!(green_f: |color: String| {
    RGBA::from_hex(&color)?.g.as_f32()
});
handlebars_helper!(blue_f: |color: String| {
    RGBA::from_hex(&color)?.b.as_f32()
});
handlebars_helper!(alpha_f: |color: String| {
    RGBA::from_hex(&color)?.a.as_f32()
});
handlebars_helper!(red_h: |color: String| {
    format!("{:02x}", RGBA::from_hex(&color)?.r.as_u8())
});
handlebars_helper!(green_h: |color: String| {
    format!("{:02x}", RGBA::from_hex(&color)?.g.as_u8())
});
handlebars_helper!(blue_h: |color: String| {
    format!("{:02x}", RGBA::from_hex(&color)?.b.as_u8())
});
handlebars_helper!(alpha_h: |color: String| {
    format!("{:02x}", RGBA::from_hex(&color)?.a.as_u8())
});

pub fn darklight(
    h: &Helper,
    _r: &Handlebars,
    ctx: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let dark = h
        .param(0)
        .ok_or_else(|| RenderError::new("Missing parameter `dark` in position 0"))?;
    let light = h
        .param(1)
        .ok_or_else(|| RenderError::new("Missing parameter `light` in position 1"))?;

    if ctx.data()["flavor"] == "latte" {
        out.write(&light.render())?;
    } else {
        out.write(&dark.render())?;
    }

    Ok(())
}

handlebars_helper!(unquote: |value: Value| {
    let content = serde_json::to_string(&value)?;
    let content = base64::engine::general_purpose::STANDARD_NO_PAD.encode(content);
    format!("{{WHISKERS:UNQUOTE:{content}}}")
});

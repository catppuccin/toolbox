use base64::Engine;
use css_colors::{Color, Ratio, HSLA, RGBA};
use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, HelperResult, Output, RenderContext,
    RenderError, RenderErrorReason,
};

use ::titlecase::titlecase as titlecase_ext;
use serde_json::Value;

use crate::parse::ColorExt;

fn hex_error(
    helper: &'static str,
    param: &'static str,
) -> impl FnOnce(crate::parse::Error) -> RenderError {
    move |_| {
        RenderErrorReason::ParamTypeMismatchForName(helper, param.to_string(), "hex".to_string())
            .into()
    }
}

handlebars_helper!(uppercase: |s: String| s.to_uppercase());
handlebars_helper!(lowercase: |s: String| s.to_lowercase());
handlebars_helper!(titlecase: |s: String| titlecase_ext(&s));
handlebars_helper!(trunc: |number: f32, places: usize| format!("{number:.places$}"));
handlebars_helper!(lighten: |color: String, weight: f32| {
    HSLA::from_hex(&color).map_err(hex_error("lighten", "0"))?.lighten(Ratio::from_f32(weight)).to_hex()
});
handlebars_helper!(darken: |color: String, weight: f32| {
    HSLA::from_hex(&color).map_err(hex_error("darken", "0"))?.darken(Ratio::from_f32(weight)).to_hex()
});
handlebars_helper!(mix: |color_a: String, color_b: String, t: f32| {
    let a = HSLA::from_hex(&color_a).map_err(hex_error("mix", "0"))?;
    let b = HSLA::from_hex(&color_b).map_err(hex_error("mix", "1"))?;
    a.mix(b, Ratio::from_f32(t)).to_hex()
});
handlebars_helper!(opacity: |color: String, amount: f32| {
    HSLA::from_hex(&color).map_err(hex_error("opacity", "0"))?.fade(Ratio::from_f32(amount)).to_hex()
});
handlebars_helper!(rgb: |color: String| {
    RGBA::from_hex(&color).map_err(hex_error("rgb", "0"))?.to_rgb().to_string()
});
handlebars_helper!(rgba: |color: String| {
    RGBA::from_hex(&color).map_err(hex_error("rgba", "0"))?.to_string()
});
handlebars_helper!(hsl: |color: String| {
    HSLA::from_hex(&color).map_err(hex_error("hsl", "0"))?.to_hsl().to_string()
});
handlebars_helper!(hsla: |color: String| {
    HSLA::from_hex(&color).map_err(hex_error("hsla", "0"))?.to_string()
});
handlebars_helper!(red_i: |color: String| {
    RGBA::from_hex(&color).map_err(hex_error("red_i", "0"))?.r.as_u8()
});
handlebars_helper!(green_i: |color: String| {
    RGBA::from_hex(&color).map_err(hex_error("green_i", "0"))?.g.as_u8()
});
handlebars_helper!(blue_i: |color: String| {
    RGBA::from_hex(&color).map_err(hex_error("blue_i", "0"))?.b.as_u8()
});
handlebars_helper!(alpha_i: |color: String| {
    RGBA::from_hex(&color).map_err(hex_error("alpha_i", "0"))?.a.as_u8()
});
handlebars_helper!(red_f: |color: String| {
    RGBA::from_hex(&color).map_err(hex_error("red_f", "0"))?.r.as_f32()
});
handlebars_helper!(green_f: |color: String| {
    RGBA::from_hex(&color).map_err(hex_error("green_f", "0"))?.g.as_f32()
});
handlebars_helper!(blue_f: |color: String| {
    RGBA::from_hex(&color).map_err(hex_error("blue_f", "0"))?.b.as_f32()
});
handlebars_helper!(alpha_f: |color: String| {
    RGBA::from_hex(&color).map_err(hex_error("alpha_f", "0"))?.a.as_f32()
});
handlebars_helper!(red_h: |color: String| {
    format!("{:02x}", RGBA::from_hex(&color).map_err(hex_error("red_h", "0"))?.r.as_u8())
});
handlebars_helper!(green_h: |color: String| {
    format!("{:02x}", RGBA::from_hex(&color).map_err(hex_error("green_h", "0"))?.g.as_u8())
});
handlebars_helper!(blue_h: |color: String| {
    format!("{:02x}", RGBA::from_hex(&color).map_err(hex_error("blue_h", "0"))?.b.as_u8())
});
handlebars_helper!(alpha_h: |color: String| {
    format!("{:02x}", RGBA::from_hex(&color).map_err(hex_error("alpha_h", "0"))?.a.as_u8())
});

pub fn darklight(
    h: &Helper,
    _r: &Handlebars,
    ctx: &Context,
    rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let dark = h
        .param(0)
        .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("darklight", 0))?;
    let light = h
        .param(1)
        .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("darklight", 1))?;

    // if we're in an each block, we have to try and get the flavor from the iteration key
    let flavor = rc
        .block()
        .and_then(|block| block.get_local_var("key"))
        .unwrap_or_else(|| &ctx.data()["flavor"]);

    if flavor == "latte" {
        out.write(&light.render())?;
    } else {
        out.write(&dark.render())?;
    }

    Ok(())
}

handlebars_helper!(unquote: |value: Value| {
    let content = serde_json::to_string(&value).map_err(RenderErrorReason::SerdeError)?;
    let content = base64::engine::general_purpose::STANDARD_NO_PAD.encode(content);
    format!("{{WHISKERS:UNQUOTE:{content}}}")
});

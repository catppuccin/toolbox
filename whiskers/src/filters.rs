use std::{
    collections::{BTreeMap, HashMap},
    io::Write,
};

use base64::Engine as _;

use crate::models::Color;

pub fn modify(
    value: &tera::Value,
    args: &HashMap<String, tera::Value>,
) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(value.clone())?;
    if let Some(hue) = args.get("hue") {
        let hue = tera::from_value(hue.clone())?;
        Ok(tera::to_value(color.mod_hue(hue))?)
    } else if let Some(saturation) = args.get("saturation") {
        let saturation = tera::from_value(saturation.clone())?;
        Ok(tera::to_value(color.mod_saturation(saturation))?)
    } else if let Some(lightness) = args.get("lightness") {
        let lightness = tera::from_value(lightness.clone())?;
        Ok(tera::to_value(color.mod_lightness(lightness))?)
    } else if let Some(opacity) = args.get("opacity") {
        let opacity = tera::from_value(opacity.clone())?;
        Ok(tera::to_value(color.mod_opacity(opacity))?)
    } else {
        Ok(value.clone())
    }
}

pub fn add(
    value: &tera::Value,
    args: &HashMap<String, tera::Value>,
) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(value.clone())?;
    if let Some(hue) = args.get("hue") {
        let hue = tera::from_value(hue.clone())?;
        Ok(tera::to_value(color.add_hue(hue))?)
    } else if let Some(saturation) = args.get("saturation") {
        let saturation = tera::from_value(saturation.clone())?;
        Ok(tera::to_value(color.add_saturation(saturation))?)
    } else if let Some(lightness) = args.get("lightness") {
        let lightness = tera::from_value(lightness.clone())?;
        Ok(tera::to_value(color.add_lightness(lightness))?)
    } else if let Some(opacity) = args.get("opacity") {
        let opacity = tera::from_value(opacity.clone())?;
        Ok(tera::to_value(color.add_opacity(opacity))?)
    } else {
        Ok(value.clone())
    }
}

pub fn sub(
    value: &tera::Value,
    args: &HashMap<String, tera::Value>,
) -> Result<tera::Value, tera::Error> {
    let color: Color = tera::from_value(value.clone())?;
    if let Some(hue) = args.get("hue") {
        let hue = tera::from_value(hue.clone())?;
        Ok(tera::to_value(color.sub_hue(hue))?)
    } else if let Some(saturation) = args.get("saturation") {
        let saturation = tera::from_value(saturation.clone())?;
        Ok(tera::to_value(color.sub_saturation(saturation))?)
    } else if let Some(lightness) = args.get("lightness") {
        let lightness = tera::from_value(lightness.clone())?;
        Ok(tera::to_value(color.sub_lightness(lightness))?)
    } else if let Some(opacity) = args.get("opacity") {
        let opacity = tera::from_value(opacity.clone())?;
        Ok(tera::to_value(color.sub_opacity(opacity))?)
    } else {
        Ok(value.clone())
    }
}

pub fn urlencode_lzma(
    value: &tera::Value,
    _args: &HashMap<String, tera::Value>,
) -> Result<tera::Value, tera::Error> {
    // encode the data with the following process:
    // 1. messagepack the data
    // 2. compress the messagepacked data with lzma (v1, preset 9)
    // 3. urlsafe base64 encode the compressed data
    let value: BTreeMap<String, tera::Value> = tera::from_value(value.clone())?;
    let packed = rmp_serde::to_vec(&value).map_err(|e| tera::Error::msg(e.to_string()))?;
    let mut options = lzma_rust::LZMA2Options::with_preset(9);
    options.dict_size = lzma_rust::LZMA2Options::DICT_SIZE_DEFAULT;
    let mut compressed = Vec::new();
    let mut writer = lzma_rust::LZMAWriter::new(
        lzma_rust::CountingWriter::new(&mut compressed),
        &options,
        true,
        false,
        Some(packed.len() as u64),
    )?;
    writer.write_all(&packed)?;
    let _ = writer.write(&[])?;
    let encoded = base64::engine::general_purpose::URL_SAFE.encode(compressed);
    Ok(tera::to_value(encoded)?)
}

#![cfg(target_family = "wasm")]

use crate::Magic;
use ril::prelude::*;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::ImageData;

fn open_rgba_image(img: &ImageData) -> Image<Rgba> {
    let data = img.data().0;
    Image::<Rgba>::from_fn(img.width(), img.height(), |x, y| {
        let i = (y * img.width() + x) as usize * 4;
        Rgba::new(data[i], data[i + 1], data[i + 2], data[i + 3])
    })
}

#[wasm_bindgen]
pub enum Layout {
    Composite,
    Stacked,
    Grid,
}

#[wasm_bindgen]
pub fn catwalk_imagedata(
    latte: ImageData,
    frappe: ImageData,
    macchiato: ImageData,
    mocha: ImageData,
    layout: Layout,
    radius: u32,
    gap: u32,
) -> Result<ImageData, JsValue> {
    let magic = Magic::new(
        [
            open_rgba_image(&latte),
            open_rgba_image(&frappe),
            open_rgba_image(&macchiato),
            open_rgba_image(&mocha),
        ],
        radius,
    )
    .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

    let buffer = match layout {
        Layout::Composite => magic.gen_composite(),
        Layout::Stacked => magic.gen_stacked(),
        Layout::Grid => magic.gen_grid(gap),
    };

    let data = buffer
        .data
        .iter()
        .flat_map(|rgba| vec![rgba.r, rgba.g, rgba.b, rgba.a])
        .collect::<Vec<u8>>();

    ImageData::new_with_u8_clamped_array(Clamped(&data), buffer.width())
}

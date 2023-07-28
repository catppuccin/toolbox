#![cfg(target_family = "wasm")]

use crate::Magic;
use ril::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::ImageData;
use wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn open_rgba_image(bytes: &Vec<u8>) -> Result<Image<Rgba>, JsError> {
    Image::<Rgba>::from_bytes_inferred(bytes).map_or(Err(JsError::new("Failed to open image")), Ok)
}

#[wasm_bindgen]
#[repr(u8)]
pub enum Layout {
    Composite,
    Stacked,
    Grid,
}

#[wasm_bindgen]
pub fn convert_images(
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
            open_rgba_image(&latte.data())?,
            open_rgba_image(&frappe.data())?,
            open_rgba_image(&macchiato.data())?,
            open_rgba_image(&mocha.data())?,
        ],
        radius,
    )
    .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

    let buffer = match layout {
        Layout::Composite => magic.gen_composite(),
        Layout::Stacked => magic.gen_stacked(),
        Layout::Grid => magic.gen_grid(gap),
    };

    let mut writebuf = Vec::new();
    let _ = buffer.encode(ImageFormat::Png, &mut writebuf);

    ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut writebuf),
        buffer.width(),
        buffer.height(),
    )
}

// needed for WASM
#![allow(clippy::use_self, clippy::missing_const_for_fn)]

use crate::{Catwalk, CatwalkError, Layout, Magic};
use ril::{Image, Rgba};
use wasm_bindgen::{prelude::*, JsValue};
#[cfg(not(feature = "wasm_buffers"))]
use web_sys::ImageData;

impl From<CatwalkError> for JsValue {
    fn from(err: CatwalkError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

#[wasm_bindgen]
impl Catwalk {
    #[cfg(not(feature = "wasm_buffers"))]
    #[wasm_bindgen(constructor)]
    pub fn new(
        latte: ImageData,
        frappe: ImageData,
        macchiato: ImageData,
        mocha: ImageData,
    ) -> Result<Catwalk, CatwalkError> {
        let images: [Image<Rgba>; 4] = [latte, frappe, macchiato, mocha]
            .into_iter()
            .map(|img| {
                let data = img.data().0;
                Image::<Rgba>::from_fn(img.width(), img.height(), |x, y| {
                    let i: usize = (y * img.width() + x) as usize * 4;
                    Rgba::new(data[i], data[i + 1], data[i + 2], data[i + 3])
                })
            })
            .collect::<Vec<Image<Rgba>>>()
            .try_into()
            .map_err(|_| CatwalkError::ReadFromBytesError)?;

        let height = images[0].height();
        let width = images[0].width();

        // verify that they're all the same size
        if images
            .iter()
            .any(|x| x.height() != height || x.width() != width)
        {
            return Err(CatwalkError::SameSize);
        };

        Ok(Self {
            images: Some(images),
            height: Some(height),
            width: Some(width),
            ..Default::default()
        })
    }

    #[cfg(feature = "wasm_buffers")]
    #[wasm_bindgen(constructor)]
    pub fn new(
        latte: Vec<u8>,
        frappe: Vec<u8>,
        macchiato: Vec<u8>,
        mocha: Vec<u8>,
    ) -> Result<Catwalk, CatwalkError> {
        let images: [Image<Rgba>; 4] = [latte, frappe, macchiato, mocha]
            .into_iter()
            .map(Image::<Rgba>::from_bytes_inferred)
            .collect::<Result<Vec<Image<Rgba>>, _>>()
            .map_err(|_| CatwalkError::ReadFromBytesError)?
            .try_into()
            .map_err(|_| CatwalkError::ReadFromBytesError)?;

        let height = images[0].height();
        let width = images[0].width();

        // verify that they're all the same size
        if images
            .iter()
            .any(|x| x.height() != height || x.width() != width)
        {
            return Err(CatwalkError::SameSize);
        };

        Ok(Self {
            images: Some(images),
            height: Some(height),
            width: Some(width),
            ..Default::default()
        })
    }

    #[cfg(feature = "wasm_buffers")]
    pub fn build(self) -> Result<js_sys::Uint8Array, CatwalkError> {
        self.prepare()?.result()
    }

    #[cfg(not(feature = "wasm_buffers"))]
    pub fn build(self) -> Result<ImageData, JsValue> {
        self.prepare()?.result()
    }
}

impl Magic {
    #[cfg(not(feature = "wasm_buffers"))]
    pub fn result(self) -> Result<ImageData, JsValue> {
        use wasm_bindgen::Clamped;
        let width = self.width;
        let data = match self.layout {
            Layout::Composite => self.gen_composite(),
            Layout::Stacked => self.gen_stacked(),
            Layout::Grid => self.gen_grid(),
        }
        .data
        .iter()
        .flat_map(|rgba| vec![rgba.r, rgba.g, rgba.b, rgba.a])
        .collect::<Vec<u8>>();
        ImageData::new_with_u8_clamped_array(Clamped(&data), width)
    }

    #[cfg(feature = "wasm_buffers")]
    pub fn result(self) -> Result<js_sys::Uint8Array, CatwalkError> {
        use ril::{encodings::png, Encoder};
        use std::io::Cursor;

        let buffer = match self.layout {
            Layout::Composite => self.gen_composite(),
            Layout::Stacked => self.gen_stacked(),
            Layout::Grid => self.gen_grid(),
        };
        let mut writebuf = Cursor::new(Vec::new());
        png::PngEncoder::encode_static(&buffer, &mut writebuf)
            .map_err(|_| CatwalkError::EncodeError)?;

        Ok(writebuf.into_inner().as_slice().into())
    }
}

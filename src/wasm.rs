// needed for WASM
#![allow(clippy::use_self, clippy::missing_const_for_fn)]

use crate::{Catwalk, CatwalkError, Magic};
use ril::{Image, Rgba};
use wasm_bindgen::{prelude::*, Clamped, JsValue};
use web_sys::ImageData;

impl From<CatwalkError> for JsValue {
    fn from(err: CatwalkError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub struct CatwalkBuffer {
    pub width: u32,
    pub height: u32,
    data: Vec<u8>,
}

#[wasm_bindgen]
impl CatwalkBuffer {
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[wasm_bindgen]
impl Catwalk {
    /// Create a new Catwalk from 4 `web_sys::ImageData` objects
    /// # Errors
    /// Returns an error if the images...
    /// - cannot be read.
    /// - are not the same size.
    pub fn new_from_imagedata(
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

    /// Create a new Catwalk from 4 `Vec<u8>`, which are in practice `Vec<[u8; 4]>` (RGBA).
    /// # Errors
    /// Returns an error if the images...
    /// - cannot be read.
    /// - are not the same size.
    pub fn new_from_u8_array(
        latte: Vec<u8>,
        frappe: Vec<u8>,
        macchiato: Vec<u8>,
        mocha: Vec<u8>,
        width: u32,
    ) -> Result<Catwalk, CatwalkError> {
        let images: [Image<Rgba>; 4] = [latte, frappe, macchiato, mocha]
            .into_iter()
            .map(|data| {
                let len = data.len() as u32;
                let height = len / (width * 4);
                Image::<Rgba>::from_fn(width, height, |x, y| {
                    let i: usize = (y * width + x) as usize * 4;
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

    /// Calculate the Catwalk image & return an `ImageData` object.
    /// # Errors
    /// Returns an error if any of `self.images`, `self.height`, or `self.width` are not set.
    pub fn build_imagedata(self) -> Result<ImageData, JsValue> {
        self.prepare()?.result()
    }

    /// Calculate the Catwalk image & return a `CatwalkBuffer` object.
    /// # Errors
    /// Returns an error if any of `self.images`, `self.height`, or `self.width` are not set.
    pub fn build_buffer(self) -> Result<CatwalkBuffer, CatwalkError> {
        Ok(self.prepare()?.result_buffer())
    }
}

impl Magic {
    /// Calculate the Catwalk image & return an `ImageData` object.
    /// # Errors
    /// Returns an error if the `ImageData` cannot be created.
    pub fn result(self) -> Result<ImageData, JsValue> {
        let width = self.width;
        let data = self
            .process()
            .data
            .iter()
            .flat_map(|rgba| vec![rgba.r, rgba.g, rgba.b, rgba.a])
            .collect::<Vec<u8>>();
        ImageData::new_with_u8_clamped_array(Clamped(&data), width)
    }

    #[must_use]
    pub fn result_buffer(self) -> CatwalkBuffer {
        let height = self.height;
        let width = self.width;
        // collect a Vec<u8> from the rgba pixels
        let data: Vec<u8> = self
            .process()
            .data
            .into_iter()
            .flat_map(|rgba| vec![rgba.r, rgba.g, rgba.b, rgba.a])
            .collect();
        CatwalkBuffer {
            width,
            height,
            data,
        }
    }
}

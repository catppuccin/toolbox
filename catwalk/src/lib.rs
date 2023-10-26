#![deny(clippy::perf, clippy::nursery, clippy::pedantic)]
// ignore u32 -> f32 & vice versa for now
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

mod mask;
use mask::{RoundMask, TrapMask};
use ril::prelude::*;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(target_family = "wasm")]
mod wasm;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(not(target_family = "wasm"), derive(clap::ValueEnum))]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub enum Layout {
    Composite,
    Stacked,
    Grid,
}

#[derive(thiserror::Error, Debug)]
pub enum CatwalkError {
    #[error("Images must be the same size")]
    SameSize,
    #[error("Builder missing argument.")]
    MissingArgument,
    #[error("Failed to read image from bytes")]
    ReadFromBytesError,
    #[error("Failed to encode image data")]
    EncodeError,

    #[error("unexpected error `{0}`")]
    JsError(String),
    #[error("unknown error from JS")]
    UnknownJsError,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub struct Catwalk {
    images: Option<[Image<Rgba>; 4]>,
    height: Option<u32>,
    width: Option<u32>,
    aa_level: u32,
    radius: u32,
    gap: u32,
    layout: Layout,
}

impl Default for Catwalk {
    fn default() -> Self {
        Self {
            images: None,
            height: None,
            width: None,
            aa_level: 15,
            radius: 3,
            gap: 50,
            layout: Layout::Composite,
        }
    }
}

#[cfg_attr(target_family = "wasm", wasm_bindgen)]
impl Catwalk {
    /// Creates a new instance of Catwalk.
    /// # Errors
    /// Returns an error if the images are not the same size.
    #[cfg(not(target_family = "wasm"))]
    pub fn new(images: [Image<Rgba>; 4]) -> Result<Self, CatwalkError> {
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

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn aa_level(mut self, aa_level: Option<u32>) -> Self {
        let Some(aa_level) = aa_level else {
            return self;
        };
        self.aa_level = aa_level;
        self
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn gap(mut self, gap: Option<u32>) -> Self {
        let Some(gap) = gap else {
            return self;
        };
        self.gap = gap;
        self
    }

    #[must_use]
    pub fn layout(mut self, layout: Option<Layout>) -> Self {
        let layout = layout.unwrap_or(self.layout);
        self.layout = layout;
        self
    }

    /// Sets the radius of the rounding mask.
    /// # Errors
    /// Returns an error if the height or width are not set (automatically inferred from the `new` method arguments)
    #[allow(clippy::use_self)]
    pub fn radius(mut self, radius: Option<u32>) -> Result<Catwalk, CatwalkError> {
        let radius = radius.unwrap_or(self.radius);

        let height = self.height.ok_or(CatwalkError::MissingArgument)?;
        let width = self.width.ok_or(CatwalkError::MissingArgument)?;

        // radius as a percentage of the image width if height > width, vice versa
        let radius = if height > width {
            (width as f32 * (radius as f32 / 100.0)) as u32
        } else {
            (height as f32 * (radius as f32 / 100.0)) as u32
        };

        self.radius = radius;
        Ok(self)
    }

    fn prepare(self) -> Result<Magic, CatwalkError> {
        let images = self.images.ok_or(CatwalkError::MissingArgument)?;
        let height = self.height.ok_or(CatwalkError::MissingArgument)?;
        let width = self.width.ok_or(CatwalkError::MissingArgument)?;

        let rounding_mask = RoundMask {
            radius: self.radius,
            aa_level: self.aa_level,
        };

        Ok(Magic {
            images,
            height,
            width,
            rounding_mask,
            gap: self.gap,
            layout: self.layout,
        })
    }

    /// Calculates the catwalk image.
    /// # Errors
    /// Returns an error if any of `self.images`, `self.height`, or `self.width` are not set.
    #[cfg(not(target_family = "wasm"))]
    pub fn build(self) -> Result<Image<Rgba>, CatwalkError> {
        let catwalk = self.prepare()?;
        Ok(catwalk.result())
    }
}

pub struct Magic {
    images: [Image<Rgba>; 4],
    height: u32,
    width: u32,
    rounding_mask: RoundMask,
    gap: u32,
    layout: Layout,
}

impl Magic {
    /// Creates the slants image.
    fn gen_composite(&self) -> Image<Rgba> {
        let w = self.width as f32;
        let h = self.height as f32;
        // Use x/y to "ground" the point later on
        let inverse_slope = -w / (4.0 * h);
        let mut masked: Vec<(Image<Rgba>, usize)> = self
            .images
            .iter()
            .enumerate()
            .map(|(i, x)| (Self::gen_mask(w, i, 2, inverse_slope).mask(x), i))
            .collect();
        masked.sort_by(|a, b| b.1.cmp(&a.1));
        let mut result = Image::new(self.width, self.height, Rgba::default())
            .with_overlay_mode(OverlayMode::Merge);
        for mask in masked.iter().as_ref() {
            result.paste(0, 0, &mask.0);
        }
        self.rounding_mask.mask(&result)
    }

    /// Creates a stacked image.
    fn gen_stacked(&self) -> Image<Rgba> {
        let gap = self.height / 3;
        let padding_x = f32::floor(3.0f32.mul_add(1.0 - gap as f32, self.width as f32)) as u32;
        let mut result = Image::new(
            (self.height * 2) + (padding_x * 3) + gap,
            self.height * 2,
            Rgba::default(),
        )
        .with_overlay_mode(OverlayMode::Merge);
        self.images
            .iter()
            .map(|x| self.rounding_mask.mask(x))
            .enumerate()
            .for_each(|(i, x)| result.paste(padding_x + (gap * (i as u32)), gap * (i as u32), &x));
        result
    }

    /// Creates a grid image.
    fn gen_grid(&self) -> Image<Rgba> {
        // Round images
        let gap = self.gap;
        let rounded: Vec<Image<Rgba>> = self
            .images
            .iter()
            .map(|x| self.rounding_mask.mask(x))
            .collect();
        // Create final
        let mut result = Image::new(
            (self.width * 2) + (gap * 3),
            (self.height * 2) + (gap * 3),
            Rgba::transparent(),
        )
        .with_overlay_mode(OverlayMode::Merge);
        // calculate the top left coordinates for each image, and paste
        rounded.iter().enumerate().for_each(|(i, img)| {
            let x = i % 2;
            let y = i / 2;
            result.paste(
                gap + (self.width + gap) * x as u32,
                gap + (self.height + gap) * y as u32,
                img,
            );
        });
        result
    }
    /// Generates a mask for the given image.
    fn gen_mask(w: f32, index: usize, aa_level: u32, inverse_slope: f32) -> TrapMask {
        if index == 3 {
            // Full mask
            return TrapMask::new(None, 0.0, aa_level);
        }
        let i = index as f32;
        let trap_top: f32 = w * i.mul_add(2.0, 3.0) / 8.0;
        // Return trapezoid mask
        // We only need to return bottom x here; we'll use the inverse slope to make a line
        TrapMask::new(Some(trap_top), inverse_slope, aa_level)
    }

    // this looks a bit odd because the WASM bindings use this as well, so
    // `result()` isn't just an oversight.
    fn process(self) -> Image<Rgba> {
        match self.layout {
            Layout::Composite => self.gen_composite(),
            Layout::Stacked => self.gen_stacked(),
            Layout::Grid => self.gen_grid(),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    #[must_use]
    pub fn result(self) -> Image<Rgba> {
        self.process()
    }
}

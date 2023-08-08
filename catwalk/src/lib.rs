mod mask;
#[cfg(target_family = "wasm")]
mod wasm;

use mask::{RoundMask, TrapMask};
use rayon::prelude::*;
use ril::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CatwalkError {
    #[error("Images must be the same size")]
    SameSize,
}

pub struct Magic {
    images: [Image<Rgba>; 4],
    height: u32,
    width: u32,
    rounding_mask: RoundMask,
}

impl Magic {
    /// Creates a new instance of Magic.
    pub fn new(images: [Image<Rgba>; 4], radius: u32) -> Result<Self, CatwalkError> {
        let height = images[0].height();
        let width = images[0].width();

        // verify that they're all the same size
        if images
            .iter()
            .any(|x| x.height() != height || x.width() != width)
        {
            return Err(CatwalkError::SameSize);
        };

        // radius as a percentage of the image width if height > width, vice versa
        let radius = if height > width {
            (width as f32 * (radius as f32 / 100.0)) as u32
        } else {
            (height as f32 * (radius as f32 / 100.0)) as u32
        };

        let rounding_mask = RoundMask {
            radius,
            aa_level: 15,
        };

        Ok(Self {
            images,
            height,
            width,
            rounding_mask,
        })
    }

    /// Creates the slants image.
    pub fn gen_composite(&self) -> Image<Rgba> {
        let w = self.width as f32;
        let h = self.height as f32;
        // Use x/y to "ground" the point later on
        let inverse_slope = -w / (4.0 * h);
        let mut masked: Vec<(Image<Rgba>, usize)> = self
            .images
            .par_iter()
            .enumerate()
            .map(|(i, x)| (Self::gen_mask(w, i, 2, inverse_slope).mask(x), i))
            .collect();
        masked.sort_by(|a, b| b.1.cmp(&a.1));
        let mut result = Image::new(self.width, self.height, Rgba::default())
            .with_overlay_mode(OverlayMode::Merge);
        for mask in masked.iter() {
            result.paste(0, 0, &mask.0);
        }
        self.rounding_mask.mask(&result)
    }

    /// Creates a stacked image.
    pub fn gen_stacked(&self) -> Image<Rgba> {
        let gap = self.height / 3;
        let padding_x = f32::floor((self.width as f32 - (3.0 * gap as f32)) / 2.0) as u32;
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

    pub fn gen_grid(&self, gap: u32) -> Image<Rgba> {
        // Round images
        let rounded: Vec<Image<Rgba>> = self
            .images
            .par_iter()
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
            )
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
        let trap_top: f32 = w * ((i * 2.0) + 3.0) / 8.0;
        // Return trapezoid mask
        // We only need to return bottom x here; we'll use the inverse slope to make a line
        TrapMask::new(Some(trap_top), inverse_slope, aa_level)
    }
}

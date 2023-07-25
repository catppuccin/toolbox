mod mask;

use crate::mask::{RoundMask, TrapMask};
use color_eyre::{eyre::eyre, Result};
use image::{ImageBuffer, Rgba, RgbaImage};
use rayon::prelude::*;

#[derive(Debug)]
pub struct Magic {
    images: [RgbaImage; 4],
    height: u32,
    width: u32,
    rounding_mask: RoundMask,
}

impl Magic {
    /// Creates a new instance of Magic.
    pub fn new(images: [RgbaImage; 4], radius: u32) -> Result<Self> {
        let height = images[0].height();
        let width = images[0].width();

        // verify that they're all the same size
        if images
            .iter()
            .any(|x| x.height() != height || x.width() != width)
        {
            return Err(eyre!("Images must be the same size"));
        };

        let rounding_mask = RoundMask { radius };

        Ok(Self {
            images,
            height,
            width,
            rounding_mask,
        })
    }

    /// Creates the slants image.
    pub fn gen_composite(&self) -> RgbaImage {
        let mut masked: Vec<(RgbaImage, usize)> = self
            .images
            .par_iter()
            .enumerate()
            .map(|(i, x)| {
                (
                    Self::gen_mask(self.height as f32, self.width as f32, i).mask(x),
                    i,
                )
            })
            .collect();
        masked.sort_by(|a, b| b.1.cmp(&a.1));
        let mut result = ImageBuffer::new(self.width, self.height);
        for mask in masked.iter() {
            image::imageops::overlay(&mut result, &mask.0, 0, 0);
        }
        self.rounding_mask.mask(&result)
    }

    // Creates a stacked image.
    pub fn gen_stacked(&self) -> RgbaImage {
        let gap = self.height / 3;
        let padding_x = f32::floor((self.width as f32 - (3.0 * gap as f32)) / 2.0) as u32;
        let mut result = RgbaImage::from_pixel(
            (self.height * 2) + (padding_x * 3) + gap,
            self.height * 2,
            Rgba([0, 0, 0, 0]),
        );
        self.images
            .iter()
            .map(|x| self.rounding_mask.mask(x))
            .enumerate()
            .for_each(|(i, x)| {
                image::imageops::overlay(
                    &mut result,
                    &x,
                    (padding_x + (gap * (i as u32))) as i64,
                    (gap * (i as u32)) as i64,
                );
            });
        result
    }

    pub fn gen_grid(&self, gap: u32) -> RgbaImage {
        // Round images
        let rounded: Vec<RgbaImage> = self
            .images
            .par_iter()
            .map(|x| self.rounding_mask.mask(x))
            .collect();
        // Create final
        let mut result = RgbaImage::from_pixel(
            (self.width * 2) + gap,
            (self.height * 2) + gap,
            Rgba([0, 0, 0, 0]),
        );
        // Paste final
        rounded.iter().enumerate().for_each(|(i, x)| {
            image::imageops::overlay(
                &mut result,
                x,
                (((i as u32) % 2) * (self.width + gap)).into(),
                (((i as u32) / 2) * (self.height + gap)).into(),
            )
        });
        result
    }
    /// Generates a mask for the given image.
    fn gen_mask(h: f32, w: f32, index: usize) -> TrapMask {
        if index == 3 {
            // Full mask
            return TrapMask::new(vec![]);
        }
        let i = index as f32;
        let trap_top: f32 = ((i * 2.0) + 3.0) / 8.0;
        let trap_btm: f32 = ((i * 2.0) + 1.0) / 8.0;
        // Return trapezoid mask
        // We only need to return the line here: the trapezoid is from top to bottom
        TrapMask::new(vec![(trap_top * w, 0.0), (trap_btm * w, h)])
    }
}

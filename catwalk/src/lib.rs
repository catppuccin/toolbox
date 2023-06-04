mod mask;
pub use clap::Parser;
use image::{open, ImageBuffer, Rgba};
use crate::mask::{Mask, MagicBuf};
use rayon::prelude::*;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Latte screenshot
    latte: Option<String>,
    /// Frappe screenshot
    frappe: Option<String>,
    /// Macchiato screenshot
    macchiato: Option<String>,
    /// Mocha screenshot
    mocha: Option<String>,
    /// Margin
    #[arg(short, long, default_value_t = 40)]
    pub margin: u32,
    /// Background Color
    #[arg(short, long, default_value_t = str::to_string("#00000000"))]
    pub background: String,
}


#[derive(Debug)]
pub struct Magic {
    images: [MagicBuf; 4],
}

impl Magic {
    /// Creates the slants image.
    pub fn gen_slants(&self) -> MagicBuf {
        let height = self.images[0].height();
        let width = self.images[0].width();
        for image in self.images.iter() {
            if image.height() != height || image.width() != width {
                panic!("All images must have the same dimensions.")
            }
        }
        let mut masked: Vec<(MagicBuf, usize)> = self.images.par_iter().enumerate().map(|(i, x)| (Self::gen_mask(height as f32, width as f32, i).mask(x), i)).collect();
        masked.sort_by(|a, b| b.1.cmp(&a.1));
        let mut result = ImageBuffer::new(width, height);
        for mask in masked.iter() {
            image::imageops::overlay(&mut result, &mask.0, 0, 0);
        }
        result
    }
    /// Generates a mask for the given image.
    fn gen_mask(h: f32, w: f32, index: usize) -> Mask {
        if index == 3 {
            // Full mask
            return Mask::new(vec![])
        }
        let i = index as f32;
        let trap_top: f32 = ((i*2.0)+3.0)/8.0;
        let trap_btm: f32 = ((i*2.0)+1.0)/8.0;
        // Return trapezoid mask
        // We only need to return the line here: the trapezoid is from top to bottom
        Mask::new(vec![(trap_top*w, 0.0), (trap_btm*w, h)])
    }
}

pub trait MagicTricks {
    fn margin(&self, m: u32, color: String) -> MagicBuf;
}
impl MagicTricks for MagicBuf {
    fn margin(&self, m: u32, color: String) -> MagicBuf {
        // Decode hex
        let mut hex_color = [0 as u8; 4];
        hex::decode_to_slice(color, &mut hex_color).expect("Please provide a valid RGBA hex.");
        let mut result = MagicBuf::from_pixel(self.width()+m, self.height()+m, Rgba(hex_color));
        image::imageops::overlay(&mut result, self, (m/2) as i64, (m/2) as i64);
        result
    }
}

/// Create a new Magic from arguments provided
impl From<Args> for Magic {
    fn from(args: Args) -> Self {
        Self {
            images: [
                open(args.latte.unwrap_or_else(|| panic!("Not enough arguments."))).unwrap_or_else(|_| panic!("Failed to open file(s).")).into_rgba8(),
                open(args.frappe.unwrap_or_else(|| panic!("Not enough arguments."))).unwrap_or_else(|_| panic!("Failed to open file(s).")).into_rgba8(),
                open(args.macchiato.unwrap_or_else(|| panic!("Not enough arguments."))).unwrap_or_else(|_| panic!("Failed to open file(s).")).into_rgba8(),
                open(args.mocha.unwrap_or_else(|| panic!("Not enough arguments."))).unwrap_or_else(|_| panic!("Failed to open file(s).")).into_rgba8(),
            ],
        }
    }
}

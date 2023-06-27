mod mask;
pub use clap::Parser;
use image::{open, ImageBuffer, Rgba};
use crate::mask::TrapMask;
pub use crate::mask::{MagicBuf, RoundMask};
use rayon::prelude::*;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
pub struct Args {
    /// Latte screenshot
    latte: Option<String>,
    /// Frappe screenshot
    frappe: Option<String>,
    /// Macchiato screenshot
    macchiato: Option<String>,
    /// Mocha screenshot
    mocha: Option<String>,
    /// Layout
    #[arg(short, long, default_value_t = str::to_string("composite"))]
    pub layout: String,
    /// Gap (grid layout)
    #[arg(short, long, default_value_t = 150)]
    pub gap: u32,
    /// Sets the radius.
    #[arg(short, long, default_value_t = 75)]
    pub radius: u32,
}


#[derive(Debug)]
pub struct Magic {
    images: [MagicBuf; 4],
}

impl Magic {
    /// Creates the slants image.
    pub fn gen_composite(&self, radius: u32) -> MagicBuf {
        let height = self.images[0].height();
        let width = self.images[0].width();
        let round = RoundMask { radius };
        self.check_heights(height, width);
        let mut masked: Vec<(MagicBuf, usize)> = self.images.par_iter()
            .enumerate()
            .map(|(i, x)| (Self::gen_mask(height as f32, width as f32, i).mask(x), i))
            .collect();
        masked.sort_by(|a, b| b.1.cmp(&a.1));
        let mut result = ImageBuffer::new(width, height);
        for mask in masked.iter() {
            image::imageops::overlay(&mut result, &mask.0, 0, 0);
        }
        round.mask(&result)
    }
    // Creates a stacked image.
    pub fn gen_stacked(&self, radius: u32) -> MagicBuf {
        let height = self.images[0].height();
        let width = self.images[0].width();
        let round = RoundMask { radius };
        self.check_heights(height, width);
        let gap = height / 3;
        let padding_x = f32::floor((width as f32 - (3.0 * gap as f32)) / 2.0) as u32;
        let mut result = MagicBuf::from_pixel((height * 2) + (padding_x * 3) + gap, height * 2, Rgba([0, 0, 0, 0]));
        self.images.iter()
            .map(|x| round.mask(&x))
            .enumerate()
            .for_each(|(i, x)| {
                image::imageops::overlay(&mut result, &x, (padding_x + (gap * (i as u32))) as i64, (gap * (i as u32)) as i64);
            });
        result
    }

    pub fn gen_grid(&self, radius: u32, gap: u32) -> MagicBuf {
        let height = self.images[0].height();
        let width = self.images[0].width();
        let round = RoundMask { radius };
        // Check heights or panic
        self.check_heights(height, width);
        // Round images
        let rounded: Vec<MagicBuf> = self.images.par_iter().map(|x| round.mask(&x)).collect();
        // Create final
        let mut result = MagicBuf::from_pixel((width * 2) + gap, (height * 2) + gap, Rgba([0, 0, 0, 0]));
        // Paste final
        rounded.iter()
            .enumerate()
            .for_each(|(i, x)| {
                image::imageops::overlay(&mut result, x, (((i as u32) % 2) * (width + gap)).into(), (((i as u32) / 2) * (height + gap)).into())
            });
        result
    }
    /// Generates a mask for the given image.
    fn gen_mask(h: f32, w: f32, index: usize) -> TrapMask {
        if index == 3 {
            // Full mask
            return TrapMask::new(vec![])
        }
        let i = index as f32;
        let trap_top: f32 = ((i*2.0)+3.0)/8.0;
        let trap_btm: f32 = ((i*2.0)+1.0)/8.0;
        // Return trapezoid mask
        // We only need to return the line here: the trapezoid is from top to bottom
        TrapMask::new(vec![(trap_top*w, 0.0), (trap_btm*w, h)])
    }

    /// Panics if all images don't match the supplied dimensions
    fn check_heights(&self, height: u32, width: u32) {
        for image in self.images.iter() {
            if image.height() != height || image.width() != width {
                panic!("All images must have the same dimensions.")
            }
        }
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

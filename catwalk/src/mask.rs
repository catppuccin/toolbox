use image::{ImageBuffer, Rgba};

pub type MagicBuf = ImageBuffer<Rgba<u8>, Vec<u8>>;

enum MaskType {
    Full,
    Partial(Vec<(f32, f32)>)
}
pub struct Mask {
    vertices: MaskType,
}

impl Mask {
    /// Construct a new shape.
    pub fn new(vertices: Vec<(f32, f32)>) -> Self {
        if vertices.is_empty() {
            return Self { vertices: MaskType::Full }
        }
        Self {
            vertices: MaskType::Partial(vertices),
        }
    }
    /// Apply mask onto given image
    pub fn mask(&self, mask: &MagicBuf) -> MagicBuf {
        match &self.vertices {
            MaskType::Full => mask.clone(),
            MaskType::Partial(v) => {
                // Use x/y to "ground" the point later on
                let inverse_slope = -1.0 * f32::abs((v[0].0 - v[1].0) / (v[0].1 - v[1].1));
                ImageBuffer::from_fn(mask.width(), mask.height(), |x, y| {
                    if ((x as f32) - ((y as f32)*inverse_slope)) <=v[0].0 {
                        // In mask
                        mask[(x, y)]
                    } else {
                        // Not in mask
                        Rgba([0, 0, 0, 0])
                    }
                })
            }
        }
    }
}

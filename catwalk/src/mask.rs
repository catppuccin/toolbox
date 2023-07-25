use image::{ImageBuffer, Rgba};

pub type MagicBuf = ImageBuffer<Rgba<u8>, Vec<u8>>;

enum MaskType {
    Full,
    Partial(Vec<(f32, f32)>)
}
pub struct TrapMask {
    vertices: MaskType,
}

pub struct RoundMask {
    pub radius: u32,
}

impl RoundMask {
    // Applies a round mask on an object
    pub fn mask(&self, mask: &MagicBuf) -> MagicBuf {
        // Save us some work
        if self.radius == 0 {
            return mask.clone();
        }
        let h = mask.height();
        let w = mask.width();
        let r = self.radius;
        // Inner corners
        let corners = [(r, r), (w - r, r), (w - r, h - r), (r, h - r)];
        MagicBuf::from_fn(w, h, |x, y| {
            if ((x <= r) || (x >= corners[2].0)) && ((y <= r) || (y >= corners[2].1)) {
                // y is in corner squares
                if corners.iter().map(|c| Self::is_dis(&(x, y), c, r)).collect::<Vec<bool>>() == vec![false, false, false, false] {
                    // y is not in rectangle
                    return Rgba([0, 0, 0, 0]);
                }
            }
            mask[(x, y)]
        })
    }
    // Checks if two points are at most r away from each other.
    fn is_dis(p1: &(u32, u32), p2: &(u32, u32), r: u32) -> bool {
        u32::abs_diff(p1.0, p2.0).pow(2) + u32::abs_diff(p1.1, p2.1).pow(2) <= r.pow(2)
    }
}

impl TrapMask {
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
                MagicBuf::from_fn(mask.width(), mask.height(), |x, y| {
                    if ((x as f32) - ((y as f32)*inverse_slope)) <= v[0].0 {
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

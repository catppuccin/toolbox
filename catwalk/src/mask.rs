use ril::prelude::*;

enum MaskType {
    Full,
    Partial(f32, f32),
}

pub struct TrapMask {
    vertices: MaskType,
    aa_level: u32,
}

#[derive(Debug)]
pub struct RoundMask {
    pub radius: u32,
    pub aa_level: u32,
}

impl RoundMask {
    /// Applies a round mask on an object
    pub fn mask(&self, image: &Image<Rgba>) -> Image<Rgba> {
        // Save us some work
        if self.radius == 0 {
            return image.clone();
        }
        let h = image.height();
        let w = image.width();
        let r = self.radius;
        // Inner corners
        let corners = [(r, r), (w - r, r), (w - r, h - r), (r, h - r)];
        Image::from_fn(w, h, |x, y| {
            if ((x <= r) || (x >= corners[2].0)) && ((y <= r) || (y >= corners[2].1)) {
                // y is in corner squares
                let distances = corners.iter().map(|c| Self::get_dis((x, y), c.to_owned()));
                if distances
                    .clone()
                    .map(|c| c <= r.pow(2))
                    .collect::<Vec<bool>>()
                    == vec![false, false, false, false]
                {
                    // y is not in mask
                    let diffs: Vec<u32> = distances.map(|dis| dis - r.pow(2)).collect();
                    for diff in diffs {
                        if diff <= self.aa_level.pow(2) {
                            // Fraction of opacity
                            let frac: f32 = 1.0 - (diff as f32 / self.aa_level.pow(2) as f32);
                            return image.pixel(x, y).with_alpha((255.0 * frac) as u8);
                        }
                    }
                    return Rgba::transparent();
                }
            }
            *image.pixel(x, y)
        })
    }

    /// Gets distance between two points
    const fn get_dis(p1: (u32, u32), p2: (u32, u32)) -> u32 {
        u32::abs_diff(p1.0, p2.0).pow(2) + u32::abs_diff(p1.1, p2.1).pow(2) //<= r.pow(2)
    }
}

impl TrapMask {
    /// Construct a new shape.
    pub fn new(vertex: Option<f32>, inverse_slope: f32, aa_level: u32) -> Self {
        Self {
            vertices: vertex.map_or(MaskType::Full, |v| MaskType::Partial(v, inverse_slope)),
            aa_level,
        }
    }

    /// Apply mask onto given image
    pub fn mask(&self, image: &Image<Rgba>) -> Image<Rgba> {
        match &self.vertices {
            MaskType::Full => image.clone(),
            MaskType::Partial(v, inverse_slope) => {
                let w = image.width();
                let h = image.height();
                Image::from_fn(w, h, |x, y| {
                    let gpos = (x as f32) - ((y as f32) * inverse_slope);
                    if gpos <= *v {
                        *image.pixel(x, y)
                    } else {
                        // Not in mask
                        let diff: f32 = gpos - v;
                        if diff <= self.aa_level as f32 {
                            // Fraction of opacity
                            let frac = 1.0 - (diff / (self.aa_level as f32));
                            return image.pixel(x, y).with_alpha((255.0 * frac) as u8);
                        }
                        Rgba::transparent()
                    }
                })
            }
        }
    }
}

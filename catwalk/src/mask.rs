use ril::prelude::*;

enum MaskType {
    Full,
    Partial(Vec<(f32, f32)>),
}

pub struct TrapMask {
    vertices: MaskType,
    aa_level: u32,
}

pub struct RoundMask {
    pub radius: u32,
    pub aa_level: u32,
}

impl RoundMask {
    // Applies a round mask on an object
    pub fn mask(&self, img: &Image<Rgba>) -> Image<Rgba> {
        // Save us some work
        if self.radius == 0 {
            return img.clone();
        }
        let h = img.height() * self.aa_level;
        let w = img.width() * self.aa_level;
        let r = self.radius * self.aa_level;
        // Inner corners
        let corners = [(r, r), (w - r, r), (w - r, h - r), (r, h - r)];
        let mut mask = Image::from_fn(w, h, |x, y| {
            if ((x <= r) || (x >= corners[2].0)) && ((y <= r) || (y >= corners[2].1)) {
                // y is in corner squares
                if corners
                    .iter()
                    .map(|c| Self::is_dis(&(x, y), c, r))
                    .collect::<Vec<bool>>()
                    == vec![false, false, false, false]
                {
                    // y is not in rectangle
                    return Rgba::transparent();
                }
            }
            Rgba::white()
        });
        // downsample for anti-aliasing
        mask.resize(img.width(), img.height(), ResizeAlgorithm::Lanczos3);
        mask.save(ImageFormat::WebP, "mask.webp").unwrap();

        // Apply mask
        let mut res = img.clone();
        res.mask_alpha(&mask.bands().3);
        res
    }

    // Checks if two points are at most r away from each other.
    fn is_dis(p1: &(u32, u32), p2: &(u32, u32), r: u32) -> bool {
        u32::abs_diff(p1.0, p2.0).pow(2) + u32::abs_diff(p1.1, p2.1).pow(2) <= r.pow(2)
    }
}

impl TrapMask {
    /// Construct a new shape.
    pub fn new(vertices: Vec<(f32, f32)>, aa_level: u32) -> Self {
        Self {
            vertices: match vertices.len() {
                0 => MaskType::Full,
                _ => MaskType::Partial(vertices),
            },
            aa_level,
        }
    }

    /// Apply mask onto given image
    pub fn mask(&self, image: &Image<Rgba>) -> Image<Rgba> {
        match &self.vertices {
            MaskType::Full => image.clone(),
            MaskType::Partial(v) => {
                // Use x/y to "ground" the point later on
                let inverse_slope = -1.0 * f32::abs((v[0].0 - v[1].0) / (v[0].1 - v[1].1));
                let w = image.width() * self.aa_level;
                let h = image.height() * self.aa_level;
                let mut mask = Image::from_fn(w, h, |x, y| {
                    if ((x as f32) - ((y as f32) * inverse_slope)) <= v[0].0 {
                        Rgba::white()
                    } else {
                        // Not in mask
                        Rgba::transparent()
                    }
                });
                // downsample for anti-aliasing
                mask.resize(image.width(), image.height(), ResizeAlgorithm::Lanczos3);
                let mut res = image.clone();
                res.mask_alpha(&mask.bands().3);
                res
            }
        }
    }
}

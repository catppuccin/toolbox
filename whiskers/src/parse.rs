use std::num::ParseIntError;

use css_colors::{rgba, Color, Ratio, HSLA, RGB, RGBA};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid length {0} for hex string, expected 6 or 8 characters")]
    InvalidLength(usize),

    #[error("failed to parse as base 16 integer: {0}")]
    ParseInt(ParseIntError),
}

pub trait HSLAExt {
    fn from_hex(hex: &str) -> Result<HSLA, Error>;
    fn to_hex(&self) -> String;
}

fn hex_to_u8s(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

impl HSLAExt for HSLA {
    fn from_hex(hex: &str) -> Result<Self, Error> {
        if hex.len() != 6 && hex.len() != 8 {
            return Err(Error::InvalidLength(hex.len()));
        }

        let components = hex_to_u8s(hex).map_err(Error::ParseInt)?;
        let [red, green, blue]: [u8; 3] = components[..3]
            .try_into()
            .expect("guaranteed to have at least 3 elements");
        let alpha = components.get(3).copied().unwrap_or(255);

        Ok(rgba(red, green, blue, Ratio::from_u8(alpha).as_f32()).to_hsla())
    }

    fn to_hex(&self) -> String {
        if self.a.as_u8() == 255 {
            let RGB { r, g, b } = self.to_rgb();
            format!("{:02x}{:02x}{:02x}", r.as_u8(), g.as_u8(), b.as_u8())
        } else {
            let RGBA { r, g, b, a } = self.to_rgba();
            format!(
                "{:02x}{:02x}{:02x}{:02x}",
                r.as_u8(),
                g.as_u8(),
                b.as_u8(),
                a.as_u8()
            )
        }
    }
}

#[cfg(test)]
mod tests {}

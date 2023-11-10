#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::cast_possible_truncation)] // we like truncating u32s into u8s around here
pub mod frontmatter;
mod helper;
mod parse;
pub mod postprocess;
pub mod template;

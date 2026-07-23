//! Contains types elevated from submodules 
//! which are shared across the project (especially between module `project` and the others) 
//! and simple enough without complexity in elevation process.

use serde::{Serialize, Deserialize};

/// Allowed pixel values in `MaskLayer` and `FinalMask`.
#[repr(u8)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum MaskValue {
    Transparent = 0,
    Include = 1,
    Exclude = 2
}

impl TryFrom<u8> for MaskValue {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Transparent),
            1 => Ok(Self::Include),
            2 => Ok(Self::Exclude),
            _ => Err(())
        }
    }
}

/// Allowed shapes for `VectorMask`.
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum VectorShape {
    Rect {
        x: usize,
        y: usize,
        w: usize,
        h: usize
    },
    Path {
        radius: usize,
        points: Vec<(usize, usize)>
    }
}
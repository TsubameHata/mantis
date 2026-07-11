use image::{GrayImage};

use crate::split::mask::{MaskLayer, MaskValue};

pub enum VectorShape {
    Rect {
        x: usize,
        y: usize,
        w: usize,
        h: usize
    },
    Path {
        thickness: usize,
        points: Vec<(usize, usize)>
    }
}

pub struct VectorMask {
    pub value: MaskValue,
    pub shape: VectorShape
}

/// For internal use only.
/// 
/// `mw` `mh` mean the width and height of the mask.
fn render_rect(x: usize, y: usize, w: usize, h: usize, value: MaskValue, mw: usize, mh: usize) -> GrayImage {
    // do not handle overflow, directly panics
    // may change idea about this later
    assert!(x+w<=mw && y+h<=mh);    

    let mut container = vec![MaskValue::Transparent as u8; mw*mh];

    for i in y..(y+h) {
        container[(i*mw+x)..(i*mw+x+w)].fill(value as u8);
    }

    GrayImage::from_raw(mw as u32, mh as u32, container).unwrap()
}

impl VectorMask {
    /// Render the `VectorMask` into a `MaskLayer` with given width and height of the intended mask.
    pub fn render(&self, w: usize, h: usize) -> MaskLayer {
        MaskLayer(
            match &self.shape {
                VectorShape::Rect { x, y, w: w_, h: h_ } => {
                    render_rect(*x, *y, *w_, *h_, self.value, w, h)
                },
                VectorShape::Path { thickness: _, points: _ } => todo!()
            }
        )
    }
}
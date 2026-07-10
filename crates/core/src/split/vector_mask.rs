use crate::split::mask::{MaskValue};

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
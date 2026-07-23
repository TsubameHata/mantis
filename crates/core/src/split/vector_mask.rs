use image::{GrayImage, Luma};
use imageproc::drawing::draw_filled_circle_mut;

use crate::types::MaskValue;
use crate::split::mask::MaskLayer;

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

fn paint_segment(img: &mut GrayImage, radius: usize, value: MaskValue, x1: usize, y1: usize, x2: usize, y2: usize) {
    let color = Luma([value as u8]);

    let mut draw = |x: usize, y: usize| {
        draw_filled_circle_mut(img, (x as i32, y as i32), radius as i32, color);
    };

    let dx = x2 as i32 - x1 as i32;
    let dy = y2 as i32 - y1 as i32;

    let len = ((dx*dx + dy*dy) as f32).sqrt().ceil() as usize;
    
    if len==0 { 
        draw(x1, y1);
        return; 
    }

    draw(x2, y2);

    for s in 0..len {
        let t = s as f32 / len as f32;
        let x = (x1 as f32 + t*(dx as f32)) as usize;
        let y = (y1 as f32 + t*(dy as f32)) as usize;
        draw(x, y);
    }
}

fn render_path(radius: usize, path: &[(usize, usize)], value: MaskValue, mw: usize, mh: usize) -> GrayImage {
    let mut img = GrayImage::from_pixel(mw as u32, mh as u32, Luma([MaskValue::Transparent as u8]));

    // if `path.len()==0`, no problem
    // but if it is `1`, the only point will not be painted below
    if path.len()==1 {
        let (x,y) = path[0];
        draw_filled_circle_mut(&mut img, (x as i32, y as i32), radius as i32, Luma([value as u8]));
        return img;
    }

    for window in path.windows(2) {
        let ((x1, y1), (x2, y2)) = (window[0], window[1]);
        paint_segment(&mut img, radius, value, x1, y1, x2, y2);
    }

    img
}

impl VectorMask {
    /// Render the `VectorMask` into a `MaskLayer` with given width and height of the intended mask.
    pub fn render(&self, w: usize, h: usize) -> MaskLayer {
        MaskLayer(
            match &self.shape {
                VectorShape::Rect { x, y, w: w_, h: h_ } => {
                    render_rect(*x, *y, *w_, *h_, self.value, w, h)
                },
                VectorShape::Path { radius, points } => {
                    render_path(*radius, points, self.value, w, h)
                }
            }
        )
    }
}
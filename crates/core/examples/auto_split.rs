use image::ImageReader;
use std::env;

use mantis::analysis::gaussian_conv;
use mantis::types::{MaskValue, VectorShape, Overflow};
use mantis::split::{
    vector_mask::VectorMask, 
    compose::ImageComposer,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len()==2);

    let img_raw = ImageReader::open(&args[1]).unwrap().decode().unwrap();
    let img_gray = img_raw.to_luma8();
    let img = img_raw.to_rgb8();
    let prob = gaussian_conv::prob(&img_gray);
    let peaks = gaussian_conv::find_peaks(&prob, (img.height()/8) as usize, -1f32);

    let split: Vec<_> = std::iter::once(0usize)
        .chain(peaks.into_iter())
        .chain(std::iter::once((img.height()-1) as usize))
        .collect();

    for (index, r) in split.windows(2).enumerate() {
        let (beg, end) = (r[0], r[1]);

        let w = img.width() as usize;
        let h = end - beg;

        let rect = VectorShape::Rect { x: 0, y: beg, w, h };
        let vm = VectorMask {
            value: MaskValue::Include,
            shape: rect
        };

        let mask = vm.render(img.width() as usize, img.height() as usize);
        mask.to_debug_image().save(format!("{}.mask.{}.jpg", args[1], index)).unwrap();

        let composer = ImageComposer::new()
            .mask(mask.into())
            .img(&img)
        
        // remove the following two lines to test hidden mode
            .overflow_y(Overflow::Shrink)
            .overflow_x(Overflow::Shrink)

            ;

        let out = composer.compose();
        out.save(format!("{}.result.{}.jpg", args[1], index)).unwrap();
        println!("{}", index);
    }
}
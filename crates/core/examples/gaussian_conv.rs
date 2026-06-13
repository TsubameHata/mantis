use image::ImageReader;
use std::env;

use core::analysis::gaussian_conv;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len()==2);

    let img = ImageReader::open(&args[1]).unwrap().decode().unwrap();
    let result = gaussian_conv::paint_peaks(&img);
    result.save(args[1].clone()+".result.jpg").unwrap();
}
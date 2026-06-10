use std::cmp;

use image::GrayImage;

use crate::utils;

const WHITE_THRESHOLD_PERCENTAGE: u8 = 50;
const CONV_KERNEL_SIZE_RATIO: u8 = 35;
const SIGMOID_T: f32 = 0.2;
const SIGMOID_K: f32 = 25.0;

pub fn prob(img: &GrayImage) -> Vec<f32> {
    let w = img.width() as usize;
    let h = img.height() as usize;
    
    let pixels = img.as_raw();

    // the darker the pixel is, the smaller the value is, therefore, reverse the percentage
    let white_threshold = utils::percentile_u8(pixels, 100 - WHITE_THRESHOLD_PERCENTAGE);

    let mut densities: Vec<f32> = Vec::with_capacity(h);
    for y in 0..h {
        let row = &pixels[y*w .. (y+1)*w];
        let line_density: u32 = row.iter().map(|&x| {
            let clipped = if x>=white_threshold {255u8} else {x};
            let reversed = 255-clipped;
            reversed as u32
        }).sum();
        densities.push(line_density as f32);
    }

    let kernel_size: usize = cmp::max(3usize, (h/(CONV_KERNEL_SIZE_RATIO as usize))|1);
    // uses unnormalized kernel because there is no need to normalize it here
    let kernel = utils::gaussian_kernel_unnormalized(kernel_size, (kernel_size as f32)/4f32);

    let smoothed = utils::convolve_1d_same_f32(&densities, &kernel);
    let normalized = utils::normalize_by_max_f32(&smoothed);

    let sig: Vec<f32> = normalized.iter().map(|&x| utils::sigmoid(x, SIGMOID_T, SIGMOID_K)).collect();
    let sig_normalized = utils::normalize_by_max_f32(&sig);

    sig_normalized
}
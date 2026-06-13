use std::cmp;

use image::{DynamicImage, GenericImage, GrayImage, RgbImage, Rgba};

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

/// Find peaks in probability vector.
/// 
/// Set `min_peak_distance_` to `0` to adopt default value `20`. Otherwise, `min_peak_distance >= 1`.
/// Set `min_peak_prominence_` to any negative number to adopt default value `0.03`. Otherwise, `min_peak_prominence_ >= 0`.
pub fn find_peaks(probability: &[f32], min_peak_distance_: usize, min_peak_prominence_: f32) -> Vec<usize> {
    // adopt default value
    let min_peak_distance = if min_peak_distance_==0 {20usize} else {min_peak_distance_};
    let min_peak_prominence = if min_peak_prominence_<0f32 {0.03} else {min_peak_prominence_};

    let peaks = utils::find_peaks::find_peaks(probability, min_peak_distance, min_peak_prominence);

    let centers: Vec<usize> = peaks.left_ips.iter()
        .zip(peaks.right_ips.iter())
        .map(|(&left_ip, &right_ip)| (left_ip+right_ip)/2 )
        .collect();

    centers
}

/// Calculate the result and paint the result onto the specific image.
/// 
/// It is only for internal justification, therefore the detailed mode of painting is hard-coded.
pub fn paint_peaks(img: &DynamicImage) -> DynamicImage {
    let w = img.width();
    let h = img.height();

    let probability = prob(&img.to_luma8());
    let peaks = find_peaks(&probability, (h as usize)/6usize, -1f32);

    let mut layer = DynamicImage::new_rgb8(w, h);
    for (y, l) in probability.iter().enumerate().map(|(index, &prob)| (index, (prob*(w as f32)*0.7) as usize)) {
        for x in 0..l {
            layer.put_pixel(x as u32, y as u32, Rgba([255u8, 0, 0, 0]));
        }
    }
    for &y in peaks.iter() {
        for x in 0..w {
            layer.put_pixel(x, y as u32, Rgba([0u8, 255, 0, 0]));
        }
    }

    let img_raw = img.to_rgb8().into_raw();
    let layer_raw = layer.to_rgb8().into_raw();

    let dst_raw: Vec<u8> = img_raw.iter().zip(layer_raw.iter())
        .map(|(&i, &l)| (((i as u16)+(l as u16))/2) as u8)
        .collect();

    let dst = DynamicImage::ImageRgb8(RgbImage::from_raw(w, h, dst_raw).unwrap());

    dst
}
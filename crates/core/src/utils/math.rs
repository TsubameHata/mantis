//! This module provides some mathematical operations not implemented by rust's standard library, mainly for CV calculation.

use std::f32::consts;

/// Logistic sigmoid function.
/// 
/// Sigmoid(x) = 1/(1 + exp(k(x-t)))
pub fn sigmoid(x: f32, t: f32, k: f32) -> f32 {
    1.0 / ( 1.0 + consts::E.powf(k * ( x - t)))
}

/// Find specified percentile in u8 data using counting sort.
/// 
/// 0≤`percentage`≤100.
/// 
/// It may be not consistent with the usual definition of percentile when `data.size()` is small due to algorithm.
/// 
/// ```rust
/// # use core::utils::percentile_u8;
/// # 
/// assert_eq!(percentile_u8(&[1u8, 1, 1, 2, 2, 2, 2], 60), 2);
/// 
/// // 3, 7.5 in usual definition, but 2, 7 here.
/// assert_eq!(percentile_u8(&[1u8, 2, 3, 4, 5], 50), 2);
/// assert_eq!(percentile_u8(&[6u8, 7, 8, 9], 50), 7);
/// ```
pub fn percentile_u8(data: &[u8], percentage: u8) -> u8 {
    assert!(!data.is_empty(), "data must not be empty");
    assert!(percentage<=100, "percentage out of range");

    // counting sort
    let mut bucket = [0; 256];
    for &datum in data {
        bucket[datum as usize] += 1;
    }

    // find percentile
    let threshold = (((percentage as f32) * (data.len() as f32)) / 100.0) as usize;
    let mut acc = 0usize;
    for (index, &value) in bucket.iter().enumerate() {
        acc += value as usize;
        if acc >= threshold {
            return index as u8;
        }
    }

    // fallback value
    255
}

/// Normalizes a `f32` sequence by scaling its maximum to 1.
pub fn normalize_by_max_f32(data: &[f32]) -> Vec<f32> {
    let max = data.iter().cloned().fold(0f32, f32::max);
    data.iter().map(|&x| x/max).collect()
}

/// Returns Gaussian filter coefficients. 
/// Its behavior is similar to `cv2.getGaussianKernel`, but **without normalizing by sum**.
pub fn gaussian_kernel_unnormalized(size: usize, sigma: f32) -> Vec<f32> {
    assert_eq!(size%2, 1, "kernel size must be odd");
    assert!(sigma > 0.0, "sigma must be positive");

    let unnormalized = (0..size).map(|x| {
        let i = x as f32; 
        let s = size as f32;
        consts::E.powf(-((i-(s-1.0)/2.0)/sigma).powi(2)/2.0)
    }).collect::<Vec<f32>>();

    unnormalized
}

/// 1D convolution with the same behavior as `np.convolve` with `mode="same"`.
pub fn convolve_1d_same_f32(signal: &[f32], kernel: &[f32]) -> Vec<f32> {
    let n = signal.len();
    let m = kernel.len();
    let half = m/2;
    let mut result: Vec<f32> = Vec::with_capacity(n);

    for i in 0..n {
        let mut acc = 0f32;
        for j in 0..m {
            let pos = i + j;
            if pos>=half && pos-half<n {
                acc += signal[pos-half]*kernel[j];
            }
        }
        result.push(acc);
    }

    result
}

pub fn select_by_indices<T: Copy>(arr: &[T], indices: &[usize]) -> Vec<T> {
    indices.iter().map(|&index| arr[index]).collect()
}
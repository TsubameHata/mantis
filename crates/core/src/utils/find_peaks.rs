//! A library for finding peaks inside a signal, ported from parts of `scipy.signal`.

use std::cmp;

use ordered_float::OrderedFloat;

use crate::utils;

/// Finds local maxima in `x`.
/// 
/// Ported from `scipy.signal._peak_finding_utils._local_maxima_1d`.
fn local_maxima_1d(x: &[f32]) -> Vec<usize> {
    let size = x.len();
    let half_size = size/2;

    let mut midpoints: Vec<usize> = Vec::with_capacity(half_size);
    // let mut left_edges: Vec<usize> = Vec::with_capacity(half_size);
    // let mut right_edges: Vec<usize> = Vec::with_capacity(half_size);

    let mut i = 1;
    let i_max = size - 1;

    while i < i_max {
        if x[i-1] < x[i] {
            let mut i_ahead = i + 1;
            while (i_ahead < i_max) && (x[i_ahead]==x[i]) {
                i_ahead += 1;
            }
            if x[i_ahead] < x[i] {
                // left_edges.push(i);
                // right_edges.push(i_ahead - 1);
                midpoints.push((i+i_ahead-1)/2);
                i = i_ahead;
            }
        }
        i += 1;
    }

    midpoints
}
/// Evaluate which peaks fulfill the distance condition.
/// 
/// Ported from `scipy.signal._peak_finding_utils._select_by_peak_distance`.
fn select_by_peak_distance(peaks: &[usize], x: &[f32], distance: usize) -> Vec<bool> {
    let size = peaks.len();

    let mut keep = vec![true; size];

    // priority_to_position = np.argsort(priority)
    // x is in place of priority:=x[peaks] here without numpy syntax
    let mut priority_to_position = (0..size)
        .collect::<Vec<usize>>();
    priority_to_position.sort_by_key(|&i| OrderedFloat(x[peaks[i]]));
    // change the mutibility
    let priority_to_position = priority_to_position;

    for i in (0..size).rev() {
        let j = priority_to_position[i];
        if keep[j] == false {continue;}

        let mut k = j;
        // 1 is added to k here different from cython version
        // to avoid problems caused by usize
        while k>0 && peaks[j]-peaks[k-1] < distance {
            keep[k-1] = false;
            k -= 1;
        }

        // avoid warn(unused_assignments)
        let mut k = j + 1;
        // original definition of k, in contrast
        while k < size && peaks[k] - peaks[j] < distance {
            keep[k] = false;
            k += 1;
        }
    }

    keep
}

struct Prominences {
    prominences: Vec<f32>,
    left_bases: Vec<usize>,
    right_bases: Vec<usize>
}

/// Calculate the prominence of each peak in the signal.
/// 
/// Ported from `scipy.signal._peak_finding_utils._peak_prominences`. 
/// `wlen` in cython version is always considered as default value `-1` here.
fn peak_prominences(peaks: &[usize], x: &[f32]) -> Prominences {
    let size = peaks.len();

    let mut prominences: Vec<f32> = Vec::with_capacity(size);
    let mut left_bases: Vec<usize> = Vec::with_capacity(size);
    let mut right_bases: Vec<usize> = Vec::with_capacity(size);

    for &peak in peaks.iter() {
        let i_min = 0usize;
        let i_max = x.len() - 1;
        assert!(i_min<=peak && i_max>=peak);

        // code about wlen is ignored here because not needed in project

        let mut i = peak;
        let mut left_min = x[peak];
        while i_min<=i && x[i]<=x[peak] {
            if x[i]<left_min {
                left_min = x[i];
            }
            i -= 1;
        }
        left_bases.push(i + 1);

        let mut i = peak;
        let mut right_min = x[peak];
        while i<=i_max && x[i]<=x[peak] {
            if x[i]<right_min {
                right_min = x[i];
            }
            i += 1;
        }
        right_bases.push(i - 1);

        let OrderedFloat(bottom) = cmp::max(OrderedFloat(left_min), OrderedFloat(right_min));
        prominences.push(x[peak] - bottom);

        // peaks may have a prominence of 0, which will cause a warning in the original code
    }

    Prominences { prominences, left_bases, right_bases }
}

struct Widths {
    widths: Vec<usize>,
    left_ips: Vec<usize>,
    right_ips: Vec<usize>
}

/// Calculate the width of each peak in a signal.
/// 
/// Ported from `scipy.signal._peak_finding_utils._peak_widths`.
/// The default value of `rel_height` is `0.5` in original code.
fn peak_widths(peaks: &[usize], x: &[f32], rel_height: f32, prominences: &[f32], left_bases: &[usize], right_bases: &[usize]) -> Widths {
    assert!(rel_height>=0f32);
    assert!(prominences.len()==left_bases.len() && left_bases.len()==right_bases.len());

    let size = prominences.len();
    
    // width_heights is not needed for this very project, therefore removed
    // let mut width_heights: Vec<f32> = Vec::with_capacity(size);

    // the original code interpolates when the true intersection height is between samples
    // not needed here, therefore the process is ignored and the definition is slightly different
    let mut widths: Vec<usize> = Vec::with_capacity(size);
    let mut left_ips: Vec<usize> = Vec::with_capacity(size);
    let mut right_ips: Vec<usize> = Vec::with_capacity(size);

    for p in 0..size {
        let i_min = left_bases[p];
        let i_max = right_bases[p];
        let peak = peaks[p];

        assert!(i_min<=peak && peak<=i_max && i_max<=size);

        let height = x[peak] - prominences[p] * rel_height;
        // width_heights.push(height);

        let mut i = peak;
        while i_min<i && height<x[i] {
            i -= 1;
        }
        // interpolation is ignored
        left_ips.push(i);

        let mut i = peak;
        while i<i_max && height<x[i] {
            i += 1;
        }
        right_ips.push(i);
        widths.push(right_ips[p]-left_ips[p]);
    }

    Widths { widths, left_ips, right_ips }
}

pub struct Peaks {
    pub peaks: Vec<usize>,
    pub prominences: Vec<f32>,
    pub left_bases: Vec<usize>,
    pub right_bases: Vec<usize>,
    pub widths: Vec<usize>,
    pub left_ips: Vec<usize>,
    pub right_ips: Vec<usize>
}

/// Find peaks inside a signal based on peak properties.
/// 
/// Ported from `scipy.signal._peak_finding.find_peaks`. 
/// Only the needed parts of the function in the project have been ported.
pub fn find_peaks(x: &[f32], distance: usize, min_prominence: f32) -> Peaks {
    assert!(distance>=1);

    let maxima = local_maxima_1d(x);

    let keep_bool = select_by_peak_distance(&maxima, &x, distance);
    let keep: Vec<usize> = (0..maxima.len())
        .filter(|&i| keep_bool[i])
        .collect();
    let peaks = utils::select_by_indices(&maxima, &keep);

    let Prominences { 
        prominences, 
        left_bases, 
        right_bases 
    } = peak_prominences(&peaks, x);
    let keep: Vec<usize> = prominences.iter().enumerate()
        .filter(|(_, &prominence)| { prominence>=min_prominence })
        .map(|(index, _)| index)
        .collect();
    let peaks = utils::select_by_indices(&peaks, &keep);
    let left_bases = utils::select_by_indices(&left_bases, &keep);
    let right_bases = utils::select_by_indices(&right_bases, &keep);

    let Widths { 
        widths, 
        left_ips, 
        right_ips } = peak_widths(&peaks, &x, 0.5f32, &prominences, &left_bases, &right_bases);
    
    Peaks { peaks, prominences, left_bases, right_bases, widths, left_ips, right_ips }
}
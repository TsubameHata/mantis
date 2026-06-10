//! A library for finding peaks inside a signal, ported from parts of `scipy.signal`.

use ordered_float::OrderedFloat;

pub struct Maxima {
    midpoints: Vec<usize>,
    left_edges: Vec<usize>,
    right_edges: Vec<usize>
}

/// Finds local maxima in `x`.
/// 
/// Ported from `scipy.signal._peak_finding_utils._local_maxima_1d`.
fn local_maxima_1d(x: &[f32]) -> Maxima {
    let size = x.len();
    let half_size = size/2;

    let mut midpoints: Vec<usize> = Vec::with_capacity(half_size);
    let mut left_edges: Vec<usize> = Vec::with_capacity(half_size);
    let mut right_edges: Vec<usize> = Vec::with_capacity(half_size);

    let mut i = 1;
    let i_max = size - 1;

    while i < i_max {
        if x[i-1] < x[i] {
            let mut i_ahead = i + 1;
            while (i_ahead < i_max) && (x[i_ahead]==x[i]) {
                i_ahead += 1;
            }
            if x[i_ahead] < x[i] {
                left_edges.push(i);
                right_edges.push(i_ahead - 1);
                midpoints.push((i+i_ahead-1)/2);
                i = i_ahead;
            }
        }
        i += 1;
    }

    Maxima {
        midpoints,
        left_edges,
        right_edges
    }
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

/// Calculate the prominence of each peak in the signal.
/// 
/// Ported from `scipy.signal._peak_finding_utils._peak_prominences`. 
/// `wlen` in cython version is always considered as default value `-1` here.
fn peak_prominences(peaks: &[usize], x: &[f32]) {
    let size = peaks.len();

    let mut prominences: Vec<f32> = Vec::with_capacity(size);
    let mut left_bases: Vec<f32> = Vec::with_capacity(size);
    let mut right_bases: Vec<f32> = Vec::with_capacity(size);

    // note: original code is at
    // https://github.com/scipy/scipy/blob/main/scipy/signal/_peak_finding_utils.pyx
    // line 167
    todo!();
}

/// Find peaks inside a signal based on peak properties.
/// 
/// Ported from `scipy.signal._peak_finding.find_peaks`. 
/// Only the needed parts of the function in the project have been ported.
pub fn find_peaks(x: &[f32], distance: usize, prominence: f32) {
    // the same message as in python source
    assert!(distance>=1, "`distance` must be greater or equal to 1");

    let Maxima {
        midpoints,
        left_edges: _,
        right_edges: _
    } = local_maxima_1d(x);

    let keep = select_by_peak_distance(&midpoints, &x, distance);
    let peaks: Vec<usize> = midpoints.iter()
        .zip(keep.iter())
        .filter(|&(_, &is_kept)| is_kept)
        .map(|(&p, &_)| p)
        .collect();

    // note: original code is at
    // https://github.com/scipy/scipy/blob/main/scipy/signal/_peak_finding.py
    // line 729
    todo!();

}
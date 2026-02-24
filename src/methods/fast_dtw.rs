use std::collections::{HashMap, HashSet};
use std::ops::Add;

use crate::traits::{Distance, Midpoint, Solution};

/// Result of a FastDTW computation. Implements [`Solution`].
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FastDtwSolution<D> {
    dist: D,
    warping_path: Vec<(usize, usize)>,
}

impl<D: Clone> Solution<D> for FastDtwSolution<D> {
    fn distance(&self) -> D {
        self.dist.clone()
    }

    fn path(&self) -> Vec<(usize, usize)> {
        self.warping_path.clone()
    }
}

fn reduce_by_half<T>(x: &[T], coarsen: &dyn Fn(&T, &T) -> T) -> Vec<T> {
    let end = x.len() - x.len() % 2;
    (0..end)
        .step_by(2)
        .map(|i| coarsen(&x[i], &x[i + 1]))
        .collect()
}

fn expand_window(
    path: &[(usize, usize)],
    len_x: usize,
    len_y: usize,
    radius: usize,
) -> Vec<(usize, usize)> {
    // Expand path cells by radius in all directions (using i64 to handle negatives)
    let mut path_set: HashSet<(i64, i64)> = HashSet::new();
    let r = radius as i64;
    for &(pi, pj) in path {
        let pi = pi as i64;
        let pj = pj as i64;
        for a in -r..=r {
            for b in -r..=r {
                path_set.insert((pi + a, pj + b));
            }
        }
    }

    // Double coords → 4 fine cells per coarse cell, filter to valid range
    let mut window_set: HashSet<(usize, usize)> = HashSet::new();
    for &(i, j) in &path_set {
        for &(di, dj) in &[(0, 0), (0, 1), (1, 0), (1, 1)] {
            let fi = i * 2 + di;
            let fj = j * 2 + dj;
            if fi >= 0 && fi < len_x as i64 && fj >= 0 && fj < len_y as i64 {
                window_set.insert((fi as usize, fj as usize));
            }
        }
    }

    // Collect as sorted list, row by row with contiguous j ranges (matching Python)
    let mut window = Vec::new();
    let mut start_j: usize = 0;
    for i in 0..len_x {
        let mut new_start_j = None;
        for j in start_j..len_y {
            if window_set.contains(&(i, j)) {
                window.push((i, j));
                if new_start_j.is_none() {
                    new_start_j = Some(j);
                }
            } else if new_start_j.is_some() {
                break;
            }
        }
        if let Some(ns) = new_start_j {
            start_j = ns;
        }
    }

    window
}

fn windowed_dtw<T, D>(
    x: &[T],
    y: &[T],
    window: &[(usize, usize)],
    distance: &dyn Fn(&T, &T) -> D,
) -> (D, Vec<(usize, usize)>)
where
    D: PartialOrd + Add<Output = D> + Clone,
{
    let len_x = x.len();
    let len_y = y.len();

    // 1-based indexing with (0,0) sentinel, matching Python defaultdict approach
    // D[i,j] = (cost, prev_i, prev_j)
    let mut cost: HashMap<(usize, usize), (D, usize, usize)> = HashMap::new();

    // We need a sentinel for infinity — we'll use Option to represent it
    // and store actual values in the map. Missing entries = infinity.

    // Sentinel: D[0,0] = (zero_cost, 0, 0) — but we don't have Default for D.
    // Instead, handle (0,0) specially below.

    // Shift window to 1-based
    for &(i, j) in window {
        let i1 = i + 1;
        let j1 = j + 1;
        let dt = distance(&x[i], &y[j]);

        // Candidates: (i-1,j), (i,j-1), (i-1,j-1) in 1-based
        // Tie-breaking: first match wins with min_by (stable on first),
        // order: up (i-1,j), left (i,j-1), diag (i-1,j-1)
        if i1 == 1 && j1 == 1 {
            // First cell — cost is just dt
            cost.insert((1, 1), (dt, 0, 0));
            continue;
        }

        // Match Python: compare (prev_cost + dt) not just prev_cost.
        // Floating-point addition can change tie-breaking when values are close.
        let predecessors = [
            (i1 - 1, j1),     // up
            (i1, j1 - 1),     // left
            (i1 - 1, j1 - 1), // diag
        ];

        let best = predecessors
            .iter()
            .filter_map(|&(pi, pj)| {
                cost.get(&(pi, pj))
                    .map(|(c, _, _)| (c.clone() + dt.clone(), pi, pj))
            })
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        if let Some((cell_cost, pi, pj)) = best {
            cost.insert((i1, j1), (cell_cost, pi, pj));
        }
    }

    // Backtrack
    let mut path = Vec::new();
    let (mut i, mut j) = (len_x, len_y);
    while i != 0 || j != 0 {
        path.push((i - 1, j - 1));
        let entry = &cost[&(i, j)];
        let (pi, pj) = (entry.1, entry.2);
        i = pi;
        j = pj;
    }
    path.reverse();

    let final_dist = cost[&(len_x, len_y)].0.clone();
    (final_dist, path)
}

fn fastdtw_recursive<T, D>(
    x: &[T],
    y: &[T],
    radius: usize,
    distance: &dyn Fn(&T, &T) -> D,
    coarsen: &dyn Fn(&T, &T) -> T,
) -> (D, Vec<(usize, usize)>)
where
    D: PartialOrd + Add<Output = D> + Clone,
{
    let min_time_size = radius + 2;

    if x.len() < min_time_size || y.len() < min_time_size {
        // Base case: full DTW (all cells in window)
        let window: Vec<(usize, usize)> = (0..x.len())
            .flat_map(|i| (0..y.len()).map(move |j| (i, j)))
            .collect();
        return windowed_dtw(x, y, &window, distance);
    }

    let x_shrinked = reduce_by_half(x, coarsen);
    let y_shrinked = reduce_by_half(y, coarsen);
    let (_distance, path) = fastdtw_recursive(&x_shrinked, &y_shrinked, radius, distance, coarsen);
    let window = expand_window(&path, x.len(), y.len(), radius);
    windowed_dtw(x, y, &window, distance)
}

/// Computes an approximate DTW using FastDTW with custom distance and coarsening functions.
///
/// This is the same as [`fastdtw`] but accepts closures for both element-wise
/// distance computation and sequence coarsening (averaging adjacent pairs).
///
/// # Arguments
///
/// * `radius` — Controls the size of the neighborhood around the projected path.
///   Larger values improve accuracy at the cost of speed.
/// * `distance` — A closure `(&T, &T) -> D` computing the distance between two elements.
/// * `coarsen` — A closure `(&T, &T) -> T` computing the midpoint of two elements,
///   used when halving the sequences during recursive coarsening.
///
/// # Examples
///
/// ```
/// use dtw_rs::{fastdtw_with_distance, Solution};
///
/// let x = [1.0_f64, 3.0, 9.0, 2.0, 1.0];
/// let y = [2.0_f64, 0.0, 0.0, 8.0, 7.0, 2.0];
///
/// let result = fastdtw_with_distance(
///     &x, &y, 1,
///     |a, b| (a - b).abs(),
///     |a, b| (a + b) / 2.0,
/// );
/// let distance: f64 = result.distance();
/// assert!(!result.path().is_empty());
/// ```
///
/// # Complexity
///
/// Approximately O(n) time and space for well-behaved inputs, where n is the
/// length of the longer sequence.
pub fn fastdtw_with_distance<T, D>(
    x: &[T],
    y: &[T],
    radius: usize,
    distance: impl Fn(&T, &T) -> D,
    coarsen: impl Fn(&T, &T) -> T,
) -> FastDtwSolution<D>
where
    D: PartialOrd + Add<Output = D> + Clone,
{
    let (dist, warping_path) = fastdtw_recursive(x, y, radius, &distance, &coarsen);
    FastDtwSolution { dist, warping_path }
}

/// Computes an approximate DTW distance and path using the FastDTW algorithm.
///
/// FastDTW recursively coarsens both sequences, computes DTW on the reduced
/// version, then projects and refines the warping path at each level. This
/// achieves approximately linear time complexity instead of the quadratic cost
/// of standard [`dtw`](crate::dtw).
///
/// Element distances are computed using the [`Distance`] trait and coarsening
/// uses the [`Midpoint`] trait.
///
/// # Arguments
///
/// * `radius` — Controls the size of the neighborhood around the projected path.
///   Larger values improve accuracy at the cost of speed. A radius of 1 is
///   typically sufficient.
///
/// # Examples
///
/// ```
/// use dtw_rs::{fastdtw, Solution};
///
/// let x = [1.0_f64, 3.0, 9.0, 2.0, 1.0];
/// let y = [2.0_f64, 0.0, 0.0, 8.0, 7.0, 2.0];
///
/// let result = fastdtw(&x, &y, 1);
/// let distance: f64 = result.distance();
/// let path = result.path();
/// assert_eq!(path[0], (0, 0));
/// ```
///
/// # Complexity
///
/// Approximately O(n) time and space for well-behaved inputs, where n is the
/// length of the longer sequence.
pub fn fastdtw<T, D>(x: &[T], y: &[T], radius: usize) -> FastDtwSolution<D>
where
    T: Distance<Output = D> + Midpoint,
    D: PartialOrd + Add<Output = D> + Clone,
{
    fastdtw_with_distance(x, y, radius, |a, b| a.distance(b), |a, b| a.midpoint(b))
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let x = [1.0_f64, 3.0, 9.0, 2.0, 1.0];
        let y = [2.0_f64, 0.0, 0.0, 8.0, 7.0, 2.0];
        let solution: FastDtwSolution<f64> = fastdtw(&x, &y, 1);
        let json = serde_json::to_string(&solution).unwrap();
        let deserialized: FastDtwSolution<f64> = serde_json::from_str(&json).unwrap();
        assert_eq!(solution, deserialized);
    }
}

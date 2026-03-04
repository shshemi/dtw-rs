use std::ops::Add;

use crate::{Distance, Solution, matrix::Matrix};

/// Result of an Itakura parallelogram–constrained DTW computation. Implements [`Solution`].
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ItakuraParallelogramSolution<D> {
    mat: Matrix<Option<D>>,
}

impl<D: Clone + PartialOrd> Solution<D> for ItakuraParallelogramSolution<D> {
    fn distance(&self) -> D {
        let (m, n) = self.mat.shape();
        self.mat[(m - 1, n - 1)]
            .clone()
            .expect("end cell must be reachable")
    }

    fn path(&self) -> Vec<(usize, usize)> {
        let (m, n) = self.mat.shape();
        let mut path = vec![(m - 1, n - 1)];
        let (mut i, mut j) = (m - 1, n - 1);
        while i > 0 || j > 0 {
            let diag = if i > 0 && j > 0 {
                self.mat[(i - 1, j - 1)].as_ref()
            } else {
                None
            };
            let up = if i > 0 {
                self.mat[(i - 1, j)].as_ref()
            } else {
                None
            };
            let left = if j > 0 {
                self.mat[(i, j - 1)].as_ref()
            } else {
                None
            };

            let next = match (diag, up, left) {
                (Some(a), Some(b), Some(c)) => {
                    if a <= b && a <= c {
                        (i - 1, j - 1)
                    } else if b <= c {
                        (i - 1, j)
                    } else {
                        (i, j - 1)
                    }
                }
                (Some(a), Some(b), None) => {
                    if a <= b {
                        (i - 1, j - 1)
                    } else {
                        (i - 1, j)
                    }
                }
                (Some(a), None, Some(c)) => {
                    if a <= c {
                        (i - 1, j - 1)
                    } else {
                        (i, j - 1)
                    }
                }
                (None, Some(b), Some(c)) => {
                    if b <= c {
                        (i - 1, j)
                    } else {
                        (i, j - 1)
                    }
                }
                (Some(_), None, None) => (i - 1, j - 1),
                (None, Some(_), None) => (i - 1, j),
                (None, None, Some(_)) => (i, j - 1),
                (None, None, None) => panic!("no valid predecessor"),
            };
            path.push(next);
            i = next.0;
            j = next.1;
        }
        path.reverse();
        path
    }
}

/// Rescale slopes to account for aspect ratio, matching pyts's `_get_itakura_slopes`.
fn get_itakura_slopes(m: usize, n: usize, max_slope: f64) -> (f64, f64) {
    let min_slope = 1.0 / max_slope;
    let scale_max = (n as f64 - 1.0) / (m as f64 - 2.0);
    let max_slope_scaled = (max_slope * scale_max).max(1.0);
    let scale_min = (n as f64 - 2.0) / (m as f64 - 1.0);
    let min_slope_scaled = (min_slope * scale_min).min(1.0);
    (min_slope_scaled, max_slope_scaled)
}

/// Round to 2 decimal places to avoid floating-point edge cases (matching pyts).
fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn valid_column_range(
    i: usize,
    m: usize,
    n: usize,
    min_slope: f64,
    max_slope: f64,
) -> (usize, usize) {
    let i_f = i as f64;
    let m_f = (m - 1) as f64;
    let n_f = (n - 1) as f64;

    // Lower boundary (min j): max of two lines through (0,0) and (m-1,n-1)
    let lower_left = round2(min_slope * i_f);
    let lower_right = round2(max_slope * (i_f - m_f) + n_f);
    let j_min = lower_left.max(lower_right).ceil() as isize;
    let j_min = j_min.clamp(0, n_f as isize) as usize;

    // Upper boundary (max j): min of two lines through (0,0) and (m-1,n-1)
    let upper_left = round2(max_slope * i_f);
    let upper_right = round2(min_slope * (i_f - m_f) + n_f);
    let j_max = upper_left.min(upper_right).floor() as isize;
    let j_max = j_max.clamp(0, n_f as isize) as usize;

    (j_min, j_max)
}

/// Computes DTW with an Itakura parallelogram constraint using a custom distance function.
///
/// This is the same as [`itakura_parallelogram`] but accepts a closure for
/// computing element-wise distances.
///
/// # Examples
///
/// ```
/// use dtw_rs::{itakura_parallelogram_with_distance, Solution};
///
/// let x = [1.0, 3.0, 9.0, 2.0, 1.0];
/// let y = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
///
/// let result = itakura_parallelogram_with_distance(&x, &y, 2.0, |a: &f64, b: &f64| (a - b).abs());
/// let distance: f64 = result.distance();
/// assert!(!result.path().is_empty());
/// ```
///
/// # Panics
///
/// - If `max_slope < 1.0`.
/// - If `x` or `y` has fewer than 3 elements.
///
/// # Complexity
///
/// O(n * m) time in the worst case, but typically explores fewer cells than
/// unconstrained DTW due to the parallelogram constraint.
pub fn itakura_parallelogram_with_distance<T, D>(
    x: &[T],
    y: &[T],
    max_slope: f64,
    distance: impl Fn(&T, &T) -> D,
) -> ItakuraParallelogramSolution<D>
where
    D: PartialOrd + Add<Output = D> + Clone,
{
    assert!(max_slope >= 1.0, "max_slope must be >= 1.0");
    let m = x.len();
    let n = y.len();
    assert!(
        m >= 3,
        "x must have at least 3 elements for Itakura parallelogram"
    );
    assert!(
        n >= 3,
        "y must have at least 3 elements for Itakura parallelogram"
    );
    let (min_slope, max_slope) = get_itakura_slopes(m, n, max_slope);
    let mut mat: Matrix<Option<D>> = Matrix::fill(None, m, n);

    for i in 0..m {
        let (j_min, j_max) = valid_column_range(i, m, n, min_slope, max_slope);
        if j_min > j_max {
            continue;
        }
        for j in j_min..=j_max {
            let d = distance(&x[i], &y[j]);
            let min_prev = match (i, j) {
                (0, 0) => None,
                (0, j_) => mat[(0, j_ - 1)].clone(),
                (i_, 0) => mat[(i_ - 1, 0)].clone(),
                (i_, j_) => {
                    let candidates = [
                        mat[(i_ - 1, j_ - 1)].as_ref(),
                        mat[(i_ - 1, j_)].as_ref(),
                        mat[(i_, j_ - 1)].as_ref(),
                    ];
                    candidates
                        .into_iter()
                        .flatten()
                        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                        .cloned()
                }
            };
            mat[(i, j)] = match min_prev {
                Some(prev) => Some(prev + d),
                None if i == 0 && j == 0 => Some(d),
                None => None,
            };
        }
    }

    ItakuraParallelogramSolution { mat }
}

/// Computes DTW with an Itakura parallelogram constraint.
///
/// Restricts the warping path to a parallelogram-shaped region controlled by
/// `max_slope`, preventing excessive compression or stretching of the alignment.
/// Element distances are computed using the [`Distance`] trait.
///
/// # Arguments
///
/// * `max_slope` — Controls the parallelogram shape. Must be `>= 1.0`. Higher
///   values allow more warping flexibility; `1.0` is the most restrictive.
///
/// # Examples
///
/// ```
/// use dtw_rs::{itakura_parallelogram, Solution};
///
/// let x = [1.0_f64, 3.0, 9.0, 2.0, 1.0];
/// let y = [2.0_f64, 0.0, 0.0, 8.0, 7.0, 2.0];
///
/// let result = itakura_parallelogram(&x, &y, 2.0);
/// let distance: f64 = result.distance();
/// let path = result.path();
/// assert_eq!(path[0], (0, 0));
/// ```
///
/// # Panics
///
/// - If `max_slope < 1.0`.
/// - If `x` or `y` has fewer than 3 elements.
///
/// # Complexity
///
/// O(n * m) time in the worst case, but typically explores fewer cells than
/// unconstrained DTW due to the parallelogram constraint.
pub fn itakura_parallelogram<T, D>(
    x: &[T],
    y: &[T],
    max_slope: f64,
) -> ItakuraParallelogramSolution<D>
where
    T: Distance<Output = D>,
    D: PartialOrd + Add<Output = D> + Clone,
{
    itakura_parallelogram_with_distance(x, y, max_slope, |a, b| a.distance(b))
}

/// Builder for Itakura parallelogram–constrained DTW. Uses typestate to select
/// between the [`Distance`] trait and a user-supplied closure.
///
/// # Examples
///
/// ```
/// use dtw_rs::{ItakuraParallelogram, Solution};
///
/// let x = [1.0_f64, 3.0, 9.0, 2.0, 1.0];
/// let y = [2.0_f64, 0.0, 0.0, 8.0, 7.0, 2.0];
///
/// // Using the Distance trait:
/// let result = ItakuraParallelogram::new(&x, &y, 2.0).compute();
/// assert!(!result.path().is_empty());
///
/// // Using a custom distance closure:
/// let result = ItakuraParallelogram::new(&x, &y, 2.0)
///     .distance_fn(|a: &f64, b: &f64| (a - b).abs())
///     .compute();
/// assert!(!result.path().is_empty());
/// ```
pub struct ItakuraParallelogram<'a, T, Dist = ()> {
    x: &'a [T],
    y: &'a [T],
    max_slope: f64,
    dist: Dist,
}

impl<'a, T> ItakuraParallelogram<'a, T> {
    pub fn new(x: &'a [T], y: &'a [T], max_slope: f64) -> Self {
        ItakuraParallelogram { x, y, max_slope, dist: () }
    }
}

impl<'a, T> ItakuraParallelogram<'a, T, ()> {
    pub fn distance_fn<D, F: Fn(&T, &T) -> D>(self, f: F) -> ItakuraParallelogram<'a, T, F> {
        ItakuraParallelogram { x: self.x, y: self.y, max_slope: self.max_slope, dist: f }
    }

    pub fn compute<D>(self) -> ItakuraParallelogramSolution<D>
    where
        T: Distance<Output = D>,
        D: PartialOrd + Add<Output = D> + Clone,
    {
        itakura_parallelogram(self.x, self.y, self.max_slope)
    }
}

impl<'a, T, D, F: Fn(&T, &T) -> D> ItakuraParallelogram<'a, T, F> {
    pub fn compute(self) -> ItakuraParallelogramSolution<D>
    where
        D: PartialOrd + Add<Output = D> + Clone,
    {
        itakura_parallelogram_with_distance(self.x, self.y, self.max_slope, self.dist)
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let x = [1.0_f64, 3.0, 9.0, 2.0, 1.0];
        let y = [2.0_f64, 0.0, 0.0, 8.0, 7.0, 2.0];
        let solution: ItakuraParallelogramSolution<f64> = itakura_parallelogram(&x, &y, 2.0);
        let json = serde_json::to_string(&solution).unwrap();
        let deserialized: ItakuraParallelogramSolution<f64> = serde_json::from_str(&json).unwrap();
        assert_eq!(solution, deserialized);
    }
}

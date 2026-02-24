use std::ops::Add;

use crate::{Distance, Solution, matrix::Matrix};

/// Result of a Sakoe-Chiba band–constrained DTW computation. Implements [`Solution`].
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SakoeChibaSolution<D> {
    mat: Matrix<Option<D>>,
}

impl<D: Clone + PartialOrd> Solution<D> for SakoeChibaSolution<D> {
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

/// Round to 2 decimal places to avoid floating-point edge cases (matching pyts).
fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

/// Compute the valid column range for row `i` in the Sakoe-Chiba band.
///
/// Matches pyts's `sakoe_chiba_band` logic:
/// - `scale = (n-1) / (m-1)`
/// - When `n > m`: `lower = scale * i - window`, `upper = scale * i + window`
/// - When `m > n`: `lower = scale * (i - window)`, `upper = scale * (i + window)`
/// - When `m == n`: same as `n > m` case
///
/// Returns inclusive `(j_min, j_max)`.
fn valid_column_range(i: usize, m: usize, n: usize, window_size: usize) -> (usize, usize) {
    let scale = (n as f64 - 1.0) / (m as f64 - 1.0);
    let i_f = i as f64;
    let w = window_size as f64;

    let (lower, upper) = if m > n {
        // horizontal_shift = window, vertical_shift = 0
        let actual_w = w.max(0.5 / scale);
        let lower = round2(scale * (i_f - actual_w));
        let upper = round2(scale * (i_f + actual_w));
        (lower, upper)
    } else {
        // horizontal_shift = 0, vertical_shift = window
        let actual_w = if n > m { w.max(scale / 2.0) } else { w };
        let lower = round2(scale * i_f - actual_w);
        let upper = round2(scale * i_f + actual_w);
        (lower, upper)
    };

    let j_min = lower.ceil() as isize;
    let j_min = j_min.clamp(0, (n - 1) as isize) as usize;

    let j_max = upper.floor() as isize;
    let j_max = j_max.clamp(0, (n - 1) as isize) as usize;

    (j_min, j_max)
}

/// Computes DTW with a Sakoe-Chiba band constraint using a custom distance function.
///
/// This is the same as [`sakoe_chiba`] but accepts a closure for computing
/// element-wise distances.
///
/// # Examples
///
/// ```
/// use dtw_rs::{sakoe_chiba_with_distance, Solution};
///
/// let x = [1.0, 3.0, 9.0, 2.0, 1.0];
/// let y = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
///
/// let result = sakoe_chiba_with_distance(&x, &y, 1, |a: &f64, b: &f64| (a - b).abs());
/// let distance: f64 = result.distance();
/// assert!(!result.path().is_empty());
/// ```
///
/// # Panics
///
/// - If `x` or `y` has fewer than 2 elements.
/// - If `window_size >= max(len(x), len(y))`.
///
/// # Complexity
///
/// O(n * w) time, where n is the longer sequence length and w is the window size.
pub fn sakoe_chiba_with_distance<T, D>(
    x: &[T],
    y: &[T],
    window_size: usize,
    distance: impl Fn(&T, &T) -> D,
) -> SakoeChibaSolution<D>
where
    D: PartialOrd + Add<Output = D> + Clone,
{
    let m = x.len();
    let n = y.len();
    assert!(m >= 2, "x must have at least 2 elements");
    assert!(n >= 2, "y must have at least 2 elements");
    let max_ts = m.max(n);
    assert!(
        window_size < max_ts,
        "window_size must be less than max(len(x), len(y))"
    );

    let mut mat: Matrix<Option<D>> = Matrix::fill(None, m, n);

    for i in 0..m {
        let (j_min, j_max) = valid_column_range(i, m, n, window_size);
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

    SakoeChibaSolution { mat }
}

/// Computes DTW with a Sakoe-Chiba band constraint.
///
/// Restricts the warping path to stay within `window_size` of the diagonal,
/// reducing computation compared to unconstrained [`dtw`](crate::dtw). Element
/// distances are computed using the [`Distance`] trait.
///
/// # Arguments
///
/// * `window_size` — Maximum allowed deviation from the diagonal. Smaller values
///   are faster but more restrictive.
///
/// # Examples
///
/// ```
/// use dtw_rs::{sakoe_chiba, Solution};
///
/// let x = [1.0_f64, 3.0, 9.0, 2.0, 1.0];
/// let y = [2.0_f64, 0.0, 0.0, 8.0, 7.0, 2.0];
///
/// let result = sakoe_chiba(&x, &y, 1);
/// let distance: f64 = result.distance();
/// let path = result.path();
/// assert_eq!(path[0], (0, 0));
/// ```
///
/// # Panics
///
/// - If `x` or `y` has fewer than 2 elements.
/// - If `window_size >= max(len(x), len(y))`.
///
/// # Complexity
///
/// O(n * w) time, where n is the longer sequence length and w is the window size.
pub fn sakoe_chiba<T, D>(x: &[T], y: &[T], window_size: usize) -> SakoeChibaSolution<D>
where
    T: Distance<Output = D>,
    D: PartialOrd + Add<Output = D> + Clone,
{
    sakoe_chiba_with_distance(x, y, window_size, |a, b| a.distance(b))
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let x = [1.0_f64, 3.0, 9.0, 2.0, 1.0];
        let y = [2.0_f64, 0.0, 0.0, 8.0, 7.0, 2.0];
        let solution: SakoeChibaSolution<f64> = sakoe_chiba(&x, &y, 1);
        let json = serde_json::to_string(&solution).unwrap();
        let deserialized: SakoeChibaSolution<f64> = serde_json::from_str(&json).unwrap();
        assert_eq!(solution, deserialized);
    }
}

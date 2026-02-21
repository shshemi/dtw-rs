use std::ops::Add;

use crate::{Distance, Solution, matrix::Matrix};

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

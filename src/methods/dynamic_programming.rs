use std::ops::Add;

use crate::{Distance, Solution, matrix::Matrix};

/// Result of a standard DTW computation. Implements [`Solution`].
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DtwSolution<D> {
    mat: Matrix<D>,
}

impl<D: Clone + PartialOrd> Solution<D> for DtwSolution<D> {
    fn distance(&self) -> D {
        let (m, n) = self.mat.shape();
        self.mat[(m - 1, n - 1)].clone()
    }

    fn path(&self) -> Vec<(usize, usize)> {
        let (m, n) = self.mat.shape();
        let mut path = vec![(m - 1, n - 1)];
        let (mut i, mut j) = (m - 1, n - 1);
        while i > 0 || j > 0 {
            let next = match (i, j) {
                (0, j) => (0, j - 1),
                (i, 0) => (i - 1, 0),
                (i, j) => {
                    let a = &self.mat[(i - 1, j - 1)];
                    let b = &self.mat[(i - 1, j)];
                    let c = &self.mat[(i, j - 1)];
                    if a <= b && a <= c {
                        (i - 1, j - 1)
                    } else if b <= c {
                        (i - 1, j)
                    } else {
                        (i, j - 1)
                    }
                }
            };
            path.push(next);
            i = next.0;
            j = next.1;
        }
        path.reverse();
        path
    }
}

/// Computes DTW between two sequences using a custom distance function.
///
/// This is the same as [`dtw`] but accepts a closure for computing element-wise
/// distances, allowing arbitrary distance metrics (e.g., squared Euclidean).
///
/// # Examples
///
/// ```
/// use dtw_rs::{dtw_with_distance, Solution};
///
/// let x = [1.0, 3.0, 9.0, 2.0, 1.0];
/// let y = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
///
/// let result = dtw_with_distance(&x, &y, |a: &f64, b: &f64| (a - b).powi(2));
/// let distance: f64 = result.distance();
/// let path = result.path();
/// assert!(!path.is_empty());
/// ```
///
/// # Complexity
///
/// O(n * m) time and space, where n and m are the lengths of `x` and `y`.
pub fn dtw_with_distance<T, D>(x: &[T], y: &[T], distance: impl Fn(&T, &T) -> D) -> DtwSolution<D>
where
    D: PartialOrd + Add<Output = D> + Default + Clone,
{
    //
    let mut mat = Matrix::fill(Default::default(), x.len(), y.len());
    for i in 0..x.len() {
        for j in 0..y.len() {
            //
            match (i, j) {
                (0, 0) => {
                    mat[(0, 0)] = distance(&x[0], &y[0]);
                }
                (0, j) => {
                    mat[(0, j)] = mat[(0, j - 1)].clone() + distance(&x[0], &y[j]);
                }
                (i, 0) => {
                    mat[(i, 0)] = mat[(i - 1, 0)].clone() + distance(&x[i], &y[0]);
                }
                (_, _) => {
                    let d = distance(&x[i], &y[j]);
                    let a = &mat[(i - 1, j - 1)];
                    let b = &mat[(i, j - 1)];
                    let c = &mat[(i - 1, j)];
                    let min = if a < b {
                        if a < c { a } else { c }
                    } else if b < c {
                        b
                    } else {
                        c
                    }
                    .clone();
                    mat[(i, j)] = min + d;
                }
            };
        }
    }
    DtwSolution { mat }
}

/// Computes the Dynamic Time Warping distance and path between two sequences.
///
/// Uses the standard dynamic programming algorithm with no constraints on the
/// warping path. Element distances are computed using the [`Distance`] trait.
///
/// # Examples
///
/// ```
/// use dtw_rs::{dtw, Solution};
///
/// let x = [1.0_f64, 3.0, 9.0, 2.0, 1.0];
/// let y = [2.0_f64, 0.0, 0.0, 8.0, 7.0, 2.0];
///
/// let result = dtw(&x, &y);
/// let distance: f64 = result.distance();
/// let path = result.path();
/// assert_eq!(path[0], (0, 0));
/// ```
///
/// # Complexity
///
/// O(n * m) time and space, where n and m are the lengths of `x` and `y`.
pub fn dtw<T, D>(x: &[T], y: &[T]) -> DtwSolution<D>
where
    T: Distance<Output = D>,
    D: PartialOrd + Add<Output = D> + Default + Clone,
{
    dtw_with_distance(x, y, |a, b| a.distance(b))
}

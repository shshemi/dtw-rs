use std::{cmp::min, fmt::Display, ops::Add, usize};

use super::utils::Matrix;
use crate::{Algorithm, ParameterizedAlgorithm};

#[derive(Debug, PartialEq, Clone)]
/// Dynamic time warping computation using the standard dynamic programming method.
pub struct DynamicTimeWarping<D> {
    matrix: Matrix<D>,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum Restriction {
    #[default]
    None,
    Band(usize),
}

impl<D: PartialOrd + Clone + Default + Add<D, Output = D>> Algorithm<D> for DynamicTimeWarping<D> {
    fn with_closure<T>(a: &[T], b: &[T], distance: impl Fn(&T, &T) -> D) -> Self {
        let mut dp = DynamicTimeWarping::new(a.len(), b.len());
        compute_matrix(&mut dp.matrix, |i, j| distance(&a[i], &b[j]));
        dp
    }

    fn distance(&self) -> D {
        let shape = self.matrix.shape();
        self.matrix[(shape.0 - 1, shape.1 - 1)].clone()
    }

    fn path(&self) -> Vec<(usize, usize)> {
        let shape = self.matrix.shape();
        self.path_from(shape.0 - 1, shape.1 - 1)
    }
}

impl<D: PartialOrd + Clone + Default + Add<D, Output = D>> ParameterizedAlgorithm<D>
    for DynamicTimeWarping<D>
{
    type Param = Restriction;

    fn with_closure_and_param<T>(
        a: &[T],
        b: &[T],
        distance: impl Fn(&T, &T) -> D,
        hyper_parameters: Self::Param,
    ) -> Self {
        let mut dp = DynamicTimeWarping::new(a.len(), b.len());
        match hyper_parameters {
            Restriction::None => compute_matrix(&mut dp.matrix, |i, j| distance(&a[i], &b[j])),
            Restriction::Band(band) => {
                compute_matrix_restricted_band(&mut dp.matrix, band, |i, j| distance(&a[i], &b[j]))
            }
        };
        dp
    }
}

impl<D: Display> Display for DynamicTimeWarping<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dynamic programming computation matrix: {}", self.matrix)
    }
}

impl<D: PartialOrd + Clone + Default> DynamicTimeWarping<D> {
    pub fn path_from(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let shape = self.matrix.shape();
        assert!(
            i < shape.0,
            "Dimention 0 should be less than shape.0 = {}",
            shape.0
        );
        assert!(
            j < shape.1,
            "Dimention 1 should be less than shape.1 = {}",
            shape.1
        );
        compute_path(self, i, j)
    }

    fn new(i: usize, j: usize) -> Self {
        DynamicTimeWarping {
            matrix: Matrix::<D>::new(i, j),
        }
    }
}

fn compute_matrix<D: Clone + PartialOrd + Add<D, Output = D>>(
    matrix: &mut Matrix<D>,
    distance: impl Fn(usize, usize) -> D,
) {
    for i in 0..matrix.shape().0 {
        for j in 0..matrix.shape().1 {
            optimize(matrix, i, j, &distance);
        }
    }
}
fn compute_matrix_restricted_band<D: Clone + PartialOrd + Add<D, Output = D>>(
    matrix: &mut Matrix<D>,
    band: usize,
    distance: impl Fn(usize, usize) -> D,
) {
    let slope = matrix.shape().1 as f64 / matrix.shape().0 as f64;
    let offset = [
        matrix.shape().1 / matrix.shape().0 - 1,
        matrix.shape().1 / matrix.shape().0 - 1,
        band,
    ]
    .into_iter()
    .max()
    .unwrap();
    for i in 0..matrix.shape().0 {
        let s = f64::round(slope * i as f64) as usize;
        let b = if s > offset { s - offset } else { 0 };
        let e = usize::min(offset + s + 1, matrix.shape().1);
        // let e = offset + f64::round(slope * i as f64) as usize + 1;
        for j in b..e {
            optimize(matrix, i, j, &distance);
        }
    }
}

fn compute_path<D>(dtw: &DynamicTimeWarping<D>, i: usize, j: usize) -> Vec<(usize, usize)>
where
    D: PartialOrd,
{
    let mut i = i;
    let mut j = j;
    let mut v = vec![(i, j)];
    while i != 0 || j != 0 {
        if let Some((i_, j_)) = min_cost_index(&dtw.matrix, (i, j)) {
            v.push((i_, j_));
            i = i_;
            j = j_;
        } else {
            break;
        };
    }
    v.reverse();
    v
}

#[inline]
fn optimize<D>(matrix: &mut Matrix<D>, i: usize, j: usize, distance: &impl Fn(usize, usize) -> D)
where
    D: Clone + PartialOrd + Add<D, Output = D>,
{
    // let d = distance(i, j);
    matrix[(i, j)] = min_cost_index(matrix, (i, j))
        .map(|index| distance(i, j) + matrix[index].clone())
        .unwrap_or(distance(i, j));
}

fn min_cost_index<D: PartialOrd>(
    matrix: &Matrix<D>,
    index: (usize, usize),
) -> Option<(usize, usize)> {
    let (i, j) = index;
    if i != 0 && j != 0 {
        match arg_min(
            &matrix[(i - 1, j - 1)],
            &matrix[(i - 1, j)],
            &matrix[(i, j - 1)],
        ) {
            0 => Some((i - 1, j - 1)),
            1 => Some((i - 1, j)),
            2 => Some((i, j - 1)),
            _ => panic!("I dont know what to say"),
        }
    } else if i != 0 {
        Some((i - 1, j))
    } else if j != 0 {
        Some((i, j - 1))
    } else {
        None
    }
}

// #[inline]
// fn top_cost<D>(matrix: &Matrix<D>, i: usize, j: usize) -> D
// where
//     D: PartialOrd,
// {
//     matrix[(i - 1, j)]
// }

// #[inline]
// fn left_cost<D>(matrix: &Matrix<D>, i: usize, j: usize) -> D
// where
//     D: PartialOrd,
// {
//     matrix[(i, j - 1)]
// }

// #[inline]
// fn top_left_cost(matrix: &Matrix<D>, i: usize, j: usize) -> f64 {
//     matrix[(i - 1, j - 1)]
// }

// fn min(a: f64, b: f64, c: f64) -> f64 {
//     f64::min(a, f64::min(b, c))
// }

#[inline]
fn arg_min<D: PartialOrd>(a: &D, b: &D, c: &D) -> usize {
    if a > b {
        if b > c {
            2
        } else {
            1
        }
    } else if a > c {
        2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithms::{dynamic_programming::compute_matrix_restricted_band, utils::Matrix};

    use super::{compute_matrix, compute_path, DynamicTimeWarping};

    #[test]
    fn compute_matrix_with_example() {
        let a = [1.0, 3.0, 9.0, 2.0, 1.0];
        let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
        let expected_matrix = Matrix::from(
            vec![
                1.0, 2.0, 3.0, 10.0, 16.0, 17.0, 2.0, 4.0, 5.0, 8.0, 12.0, 13.0, 9.0, 11.0, 13.0,
                6.0, 8.0, 15.0, 9.0, 11.0, 13.0, 12.0, 11.0, 8.0, 10.0, 10.0, 11.0, 18.0, 17.0,
                9.0,
            ],
            5,
            6,
        );

        let mut dtw = DynamicTimeWarping::new(a.len(), b.len());
        compute_matrix(&mut dtw.matrix, |i, j| f64::abs(a[i] - b[j]));
        println!("Matrix:");
        println!("{}", dtw.matrix);
        assert!(dtw.matrix == expected_matrix);
    }

    #[test]
    fn compute_matrix_restricted_band_with_example() {
        let a = [0.0; 5];
        let b = [0.0; 5];
        let expected_matrix = Matrix::from(
            vec![
                0.0,
                0.0,
                f64::MAX,
                f64::MAX,
                f64::MAX,
                0.0,
                0.0,
                0.0,
                f64::MAX,
                f64::MAX,
                f64::MAX,
                0.0,
                0.0,
                0.0,
                f64::MAX,
                f64::MAX,
                f64::MAX,
                0.0,
                0.0,
                0.0,
                f64::MAX,
                f64::MAX,
                f64::MAX,
                0.0,
                0.0,
            ],
            5,
            5,
        );

        let mut dtw = DynamicTimeWarping::new(a.len(), b.len());
        compute_matrix_restricted_band(&mut dtw.matrix, 1, |i, j| f64::abs(a[i] - b[j]));
        // println!("{}", dtw.matrix);
        // println!("{:?}", dtw.matrix.data().iter().zip(expected_matrix.data().iter()).map(|(e1, e2)| e1 == e2).collect::<Vec<bool>>());
        println!("Matrix:");
        println!("{}", dtw.matrix);
        println!("Expectation:");
        println!("{}", expected_matrix);
        assert!(dtw.matrix == expected_matrix);
    }

    #[test]
    fn compute_path_with_example() {
        let dtw = DynamicTimeWarping {
            matrix: Matrix::from(
                vec![
                    1.0, 2.0, 3.0, 10.0, 16.0, 17.0, 2.0, 4.0, 5.0, 8.0, 12.0, 13.0, 9.0, 11.0,
                    13.0, 6.0, 8.0, 15.0, 9.0, 11.0, 13.0, 12.0, 11.0, 8.0, 10.0, 10.0, 11.0, 18.0,
                    17.0, 9.0,
                ],
                5,
                6,
            ),
        };
        let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
        let founded_path = compute_path(&dtw, 4, 5);
        assert!(expected_path == *founded_path);
    }

    fn sized_send_sync_unpin_check<T: Sized + Send + Sync + Unpin>() {}
    #[test]
    fn check_auto_traits() {
        sized_send_sync_unpin_check::<DynamicTimeWarping<f64>>()
    }
}

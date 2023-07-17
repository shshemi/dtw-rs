use std::{fmt::Display, usize};

use super::utils::Matrix;
use crate::{Algorithm, ParameterizedAlgorithm};

#[derive(Debug, PartialEq, Clone)]
pub struct DynamicTimeWarping {
    matrix: Matrix,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum Restriction {
    #[default]
    None,
    Band(usize),
}

impl Algorithm for DynamicTimeWarping {
    fn with_closure<T>(a: &[T], b: &[T], distance: impl Fn(&T, &T) -> f64) -> Self {
        let mut dp = DynamicTimeWarping::new(a.len(), b.len());
        compute_matrix(&mut dp.matrix, |i, j| distance(&a[i], &b[j]));
        dp
    }

    fn distance(&self) -> f64 {
        let shape = self.matrix.shape();
        self.matrix[(shape.0 - 1, shape.1 - 1)]
    }

    fn path(&self) -> Vec<(usize, usize)> {
        let shape = self.matrix.shape();
        self.path_from(shape.0 - 1, shape.1 - 1)
    }
}

impl ParameterizedAlgorithm for DynamicTimeWarping {
    type Param = Restriction;

    fn with_closure_and_hyper_parameters<T>(
        a: &[T],
        b: &[T],
        distance: impl Fn(&T, &T) -> f64,
        hyper_parameters: Self::Param,
    ) -> Self {
        let mut dp = DynamicTimeWarping::new(a.len(), b.len());
        match hyper_parameters {
            Restriction::None => compute_matrix(&mut dp.matrix, |i, j| distance(&a[i], &b[j])),
            Restriction::Band(band) => compute_matrix_restricted_band(&mut dp.matrix, band, |i, j| distance(&a[i], &b[j])) 
        };
        dp
    }
}

impl Display for DynamicTimeWarping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dynamic programming computation matrix: {}", self.matrix)
    }
}

impl DynamicTimeWarping {
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

    fn new(i: usize, j: usize) -> DynamicTimeWarping {
        DynamicTimeWarping {
            matrix: Matrix::new(i, j),
        }
    }
}

fn compute_matrix(matrix: &mut Matrix, distance: impl Fn(usize, usize) -> f64) {
    for i in 0..matrix.shape().0 {
        for j in 0..matrix.shape().1 {
            optimize(matrix, i, j, &distance);
        }
    }
}
fn compute_matrix_restricted_band(
    matrix: &mut Matrix,
    band: usize,
    distance: impl Fn(usize, usize) -> f64,
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

fn compute_path(dtw: &DynamicTimeWarping, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut i = i;
    let mut j = j;
    let mut v = vec![(i, j)];
    while i != 0 || j != 0 {
        let top = top_cost(&dtw.matrix, i, j);
        let left = left_cost(&dtw.matrix, i, j);
        let top_left = top_left_cost(&dtw.matrix, i, j);
        match arg_min(top_left, top, left) {
            0 => {
                i -= 1;
                j -= 1;
            }
            1 => {
                i -= 1;
            }
            2 => {
                j -= 1;
            }
            _ => unimplemented!(),
        };
        v.push((i, j));
    }
    v.reverse();
    v
}

#[inline]
fn optimize(matrix: &mut Matrix, i: usize, j: usize, distance: &impl Fn(usize, usize) -> f64) {
    let d = distance(i, j);
    let top = top_cost(matrix, i, j);
    let left = left_cost(matrix, i, j);
    let top_left = top_left_cost(matrix, i, j);
    matrix[(i, j)] = d + min(top_left, top, left);
}

#[inline]
fn top_cost(matrix: &Matrix, i: usize, j: usize) -> f64 {
    if i == 0 {
        f64::INFINITY
    } else {
        matrix[(i - 1, j)]
    }
}

#[inline]
fn left_cost(matrix: &Matrix, i: usize, j: usize) -> f64 {
    if j == 0 {
        f64::INFINITY
    } else {
        matrix[(i, j - 1)]
    }
}

#[inline]
fn top_left_cost(matrix: &Matrix, i: usize, j: usize) -> f64 {
    if i == 0 && j == 0 {
        0.0
    } else if i == 0 || j == 0 {
        f64::INFINITY
    } else {
        matrix[(i - 1, j - 1)]
    }
}

fn min(a: f64, b: f64, c: f64) -> f64 {
    f64::min(a, f64::min(b, c))
}

fn arg_min(a: f64, b: f64, c: f64) -> usize {
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
    use crate::algorithms::{
        dynamic_programming::
             compute_matrix_restricted_band,
        utils::Matrix,
    };

    use super::{compute_matrix, compute_path, DynamicTimeWarping};

    #[test]
    fn compute_matrix_with_example() {
        let a = [1.0, 3.0, 9.0, 2.0, 1.0];
        let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
        let expected_matrix = Matrix::from(
            &[
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
            &[
                0.0, 0.0, f64::MAX, f64::MAX, f64::MAX,
                0.0, 0.0, 0.0, f64::MAX, f64::MAX,
                f64::MAX, 0.0, 0.0, 0.0, f64::MAX,
                f64::MAX, f64::MAX, 0.0, 0.0, 0.0,
                f64::MAX, f64::MAX, f64::MAX, 0.0, 0.0,
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
                &[
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
        sized_send_sync_unpin_check::<DynamicTimeWarping>()
    }
}

use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T> {
    matrix: Box<[T]>,
    shape: (usize, usize),
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        assert!(
            idx.0 < self.shape.0,
            "Dimention 0 should be less than shape.0 = {}",
            self.shape.0
        );
        assert!(
            idx.1 < self.shape.1,
            "Dimention 1 should be less than shape.1 = {}",
            self.shape.1
        );
        &self.matrix[self.shape.1 * idx.0 + idx.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        assert!(
            idx.0 < self.shape.0,
            "Dimention 0 should be less than shape.0 = {}",
            self.shape.0
        );
        assert!(
            idx.1 < self.shape.1,
            "Dimention 1 should be less than shape.1 = {}",
            self.shape.1
        );
        &mut self.matrix[self.shape.1 * idx.0 + idx.1]
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                self[(i, j)].fmt(f)?;
                write!(f, " ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Matrix<T> {
    #[allow(dead_code)]
    pub fn new(i: usize, j: usize) -> Self
    where
        T: Clone + Default,
    {
        Self {
            matrix: vec![Default::default(); i * j].into_boxed_slice(),
            shape: (i, j),
        }
    }

    pub fn fill(value: T, i: usize, j: usize) -> Self
    where
        T: Clone + Default,
    {
        Self {
            matrix: vec![value; i * j].into_boxed_slice(),
            shape: (i, j),
        }
    }

    #[cfg(test)]
    pub fn from(data: Vec<T>, i: usize, j: usize) -> Self {
        assert!(data.len() == i * j);
        Self {
            matrix: data.into_boxed_slice(),
            shape: (i, j),
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn matrix_new() {
        let dtw = Matrix::<f64>::fill(0_f64, 3, 5);
        assert!(dtw.matrix.len() == 15);
        assert!(dtw.shape == (3, 5));
        assert!(dtw.matrix.iter().all(|f| *f == 0_f64));
        let dtw = Matrix::<f64>::fill(f64::MAX, 3, 5);
        assert!(dtw.matrix.len() == 15);
        assert!(dtw.shape == (3, 5));
        assert!(dtw.matrix.iter().all(|f| *f == f64::MAX));
    }

    #[test]
    fn matrix_from() {
        let matrix = Matrix::from(vec![1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 6_f64], 2, 3);
        assert!(matrix[(0, 0)] == 1_f64);
        assert!(matrix[(0, 1)] == 2_f64);
        assert!(matrix[(0, 2)] == 3_f64);
        assert!(matrix[(1, 0)] == 4_f64);
        assert!(matrix[(1, 1)] == 5_f64);
        assert!(matrix[(1, 2)] == 6_f64);
    }

    #[test]
    #[should_panic]
    fn matrix_from_invalid_size() {
        let matrix = Matrix::from(vec![1_f64, 2_f64, 3_f64, 4_f64, 5_f64], 2, 3);
        assert!(matrix[(0, 0)] == 1_f64);
        assert!(matrix[(0, 1)] == 2_f64);
        assert!(matrix[(0, 2)] == 3_f64);
        assert!(matrix[(1, 0)] == 4_f64);
        assert!(matrix[(1, 1)] == 5_f64);
    }

    #[test]
    fn matrix_access_index() {
        let dtw = Matrix {
            matrix: (1..26).map(|i| i as f64).collect(),
            shape: (5, 5),
        };
        for i in 0..dtw.shape.0 {
            for j in 0..dtw.shape.1 {
                assert!(dtw[(i, j)] == (dtw.shape.0 * i + j + 1) as f64);
            }
        }
    }

    #[test]
    #[should_panic]
    fn matrix_access_out_of_index_0() {
        let dtw = Matrix::fill(f64::MAX, 2, 3);
        assert!(f64::is_nan(dtw[(2, 0)]))
    }

    #[test]
    #[should_panic]
    fn matrix_access_out_of_index_1() {
        let dtw = Matrix::fill(f64::MAX, 2, 3);
        assert!(f64::is_nan(dtw[(0, 3)]));
    }

    #[test]
    fn matrix_assign_index() {
        const MATRIX_SIZE: usize = 5;
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                let mut dtw = Matrix::fill(f64::MAX, MATRIX_SIZE, MATRIX_SIZE);
                dtw[(i, j)] = 1.0;
                for k in 0..MATRIX_SIZE {
                    for l in 0..MATRIX_SIZE {
                        if i == k && l == j {
                            assert!(dtw[(k, l)] == 1.0);
                        } else {
                            assert!(dtw[(k, l)] == f64::MAX);
                        }
                    }
                }
            }
        }
    }

    fn sized_send_sync_unpin_check<T: Sized + Send + Sync + Unpin>() {}
    #[test]
    fn check_auto_traits() {
        sized_send_sync_unpin_check::<Matrix<f64>>();
    }
}

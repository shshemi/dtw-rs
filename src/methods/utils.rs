use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    matrix: Box<[f64]>,
    shape: (usize, usize),
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

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

impl IndexMut<(usize, usize)> for Matrix {
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

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pad = self
            .matrix
            .iter()
            .map(|f| if f64::MAX==*f {3} else {format!("{:.2}", f).len()})
            .max()
            .unwrap()
            + 1;
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                if self[(i, j)] == f64::MAX {
                    write!(f, "{: >pad$}", "inf", pad = pad)?
                } else {
                    write!(f, "{: >pad$.2}", self[(i, j)], pad = pad)?
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

impl Matrix {
    pub fn new(i: usize, j: usize) -> Self {
        Self {
            matrix: vec![f64::MAX; i * j].into_boxed_slice(),
            shape: (i, j),
        }
    }

    #[cfg(test)]
    pub fn from(data: &[f64], i: usize, j: usize) -> Self {
        assert!(data.len() == i * j);
        Self {
            matrix: Box::from(data),
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
        let dtw = Matrix::new(3, 5);
        assert!(dtw.matrix.iter().all(|f| *f == f64::MAX));
        assert!(dtw.matrix.len() == 15);
        assert!(dtw.shape == (3, 5));
    }

    #[test]
    fn matrix_from() {
        let matrix = Matrix::from(&[1_f64,2_f64,3_f64,4_f64, 5_f64, 6_f64], 2, 3);
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
        let matrix = Matrix::from(&[1_f64,2_f64,3_f64,4_f64, 5_f64], 2, 3);
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
        let dtw = Matrix::new(2, 3);
        assert!(f64::is_nan(dtw[(2, 0)]))
    }

    #[test]
    #[should_panic]
    fn matrix_access_out_of_index_1() {
        let dtw = Matrix::new(2, 3);
        assert!(f64::is_nan(dtw[(0, 3)]));
    }

    #[test]
    fn matrix_assign_index() {
        const MATRIX_SIZE: usize = 5;
        for i in 0..MATRIX_SIZE {
            for j in 0..MATRIX_SIZE {
                let mut dtw = Matrix::new(MATRIX_SIZE, MATRIX_SIZE);
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
        sized_send_sync_unpin_check::<Matrix>()
    }
}

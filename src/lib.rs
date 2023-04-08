use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

pub trait Distance {
    fn distance(&self, other: &Self) -> f64;
}

pub type DistanceClosure<T> = Box<dyn Fn(&T, &T) -> f64>;

impl DynamicTimeWarp {
    pub fn between<T: Distance>(a: &[T], b: &[T]) -> DynamicTimeWarp {
        let mut dtw = DynamicTimeWarp::new(a.len(), b.len());
        compute_matrix(&mut dtw, |i, j| a[i].distance(&b[j]));
        dtw
    }

    pub fn distance(&self) -> f64 {
        self[(self.shape.0 - 1, self.shape.1 - 1)]
    }

    pub fn path(&self) -> Vec<(usize, usize)> {
        self.path_from(self.shape.0 - 1, self.shape.1 - 1)
    }

    pub fn path_from(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        compute_path(self, i, j)
    }

    fn new(i: usize, j: usize) -> DynamicTimeWarp {
        DynamicTimeWarp {
            matrix: vec![Default::default(); i * j].into_boxed_slice(),
            shape: (i, j),
        }
    }
}

impl Index<(usize, usize)> for DynamicTimeWarp {
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

impl IndexMut<(usize, usize)> for DynamicTimeWarp {
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

impl Display for DynamicTimeWarp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pad = self
            .matrix
            .iter()
            .map(|f| format!("{:.2}", f).len())
            .max()
            .unwrap()
            + 1;
        for i in 0..self.shape.0 {
            for j in 0..self.shape.1 {
                write!(f, "{: >pad$.2}", self[(i, j)], pad = pad)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

fn compute_matrix(dtw: &mut DynamicTimeWarp, distance: impl Fn(usize, usize) -> f64) {
    for i in 0..dtw.shape.0 {
        for j in 0..dtw.shape.1 {
            let d = distance(i, j);
            let top = top_cost(dtw, i, j);
            let left = left_cost(dtw, i, j);
            let top_left = top_left_cost(dtw, i, j);
            dtw[(i, j)] = d + min(top_left, top, left);
        }
    }
}

fn compute_path(dtw: &DynamicTimeWarp, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut i = i;
    let mut j = j;
    let mut v = vec![(i, j)];
    while i != 0 || j != 0 {
        let top = top_cost(dtw, i, j);
        let left = left_cost(dtw, i, j);
        let top_left = top_left_cost(dtw, i, j);
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
fn top_cost(dtw: &DynamicTimeWarp, i: usize, j: usize) -> f64 {
    if i == 0 {
        f64::INFINITY
    } else {
        dtw[(i - 1, j)]
    }
}

#[inline]
fn left_cost(dtw: &DynamicTimeWarp, i: usize, j: usize) -> f64 {
    if j == 0 {
        f64::INFINITY
    } else {
        dtw[(i, j - 1)]
    }
}

#[inline]
fn top_left_cost(dtw: &DynamicTimeWarp, i: usize, j: usize) -> f64 {
    if i == 0 && j == 0 {
        0.0
    } else if i == 0 || j == 0 {
        f64::INFINITY
    } else {
        dtw[(i - 1, j - 1)]
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
    use crate::{compute_matrix, DynamicTimeWarp, compute_path};

    #[test]
    fn dynamic_time_warp_new() {
        let dtw = DynamicTimeWarp::new(3, 5);
        assert!(dtw.matrix.iter().all(|f| *f == 0.0));
        assert!(dtw.matrix.len() == 15);
        assert!(dtw.shape == (3, 5));
    }

    #[test]
    fn dynamic_time_warp_access_index() {
        let dtw = DynamicTimeWarp {
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
    fn dynamic_time_warp_access_out_of_index_0() {
        let dtw = DynamicTimeWarp::new(2, 3);
        assert!(f64::is_nan(dtw[(2, 0)]))
    }

    #[test]
    #[should_panic]
    fn dynamic_time_warp_access_out_of_index_1() {
        let dtw = DynamicTimeWarp::new(2, 3);
        assert!(f64::is_nan(dtw[(0, 3)]));
    }

    #[test]
    fn dynamic_time_warp_assign_index() {
        const MATRIX_SIZE: usize = 5;
        for i in 0..MATRIX_SIZE{
            for j in 0..MATRIX_SIZE {
                let mut dtw = DynamicTimeWarp::new(MATRIX_SIZE, MATRIX_SIZE);
                dtw[(i, j)] = 1.0;
                for k in 0..MATRIX_SIZE{
                    for l in 0..MATRIX_SIZE {
                        if i == k && l == j {
                            assert!(dtw[(k, l)] == 1.0);
                        } else {
                            assert!(dtw[(k, l)] == 0.0);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn compute_matrix_with_example() {
        let a = [1.0, 3.0, 9.0, 2.0, 1.0];
        let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
        let expected_matrix = [
            1.0, 2.0, 3.0, 10.0, 16.0, 17.0,
            2.0, 4.0, 5.0, 8.0, 12.0, 13.0,
            9.0, 11.0, 13.0, 6.0, 8.0, 15.0,
            9.0, 11.0, 13.0, 12.0, 11.0, 8.0,
            10.0, 10.0, 11.0, 18.0, 17.0, 9.0,
        ];

        let mut dtw = DynamicTimeWarp::new(a.len(), b.len());
        compute_matrix(&mut dtw, |i, j| f64::abs(a[i] - b[j]));
        println!("Matrix:");
        println!("{}", dtw);
        assert!(*dtw.matrix == expected_matrix);
    }

    #[test]
    fn compute_path_with_example() {
        let dtw = DynamicTimeWarp {
            matrix: Box::new([
                1.0, 2.0, 3.0, 10.0, 16.0, 17.0,
                2.0, 4.0, 5.0, 8.0, 12.0, 13.0,
                9.0, 11.0, 13.0, 6.0, 8.0, 15.0,
                9.0, 11.0, 13.0, 12.0, 11.0, 8.0,
                10.0, 10.0, 11.0, 18.0, 17.0, 9.0,
            ]),
            shape: (5, 6),
        };
        let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
        let founded_path = compute_path(&dtw, 4, 5);
        assert!(expected_path == *founded_path);
    }
}

use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

pub trait Distance {
    fn distance(&self, other: &Self) -> f64;
}

#[derive(Debug, PartialEq)]
pub struct DynamicTimeWarp {
    matrix: Box<[f64]>,
    pub shape: (usize, usize),
}

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
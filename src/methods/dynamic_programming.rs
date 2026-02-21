use std::ops::Add;

use crate::{Distance, Solution, matrix::Matrix};

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
            let next = if i == 0 {
                (0, j - 1)
            } else if j == 0 {
                (i - 1, 0)
            } else {
                let diag = &self.mat[(i - 1, j - 1)];
                let up = &self.mat[(i - 1, j)];
                let left = &self.mat[(i, j - 1)];
                if diag <= up && diag <= left {
                    (i - 1, j - 1)
                } else if up <= left {
                    (i - 1, j)
                } else {
                    (i, j - 1)
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

pub fn dtw<T, D>(x: &[T], y: &[T]) -> DtwSolution<D>
where
    T: Distance<Output = D>,
    D: PartialOrd + Add<Output = D> + Default + Clone,
{
    dtw_with_distance(x, y, |a, b| a.distance(b))
}

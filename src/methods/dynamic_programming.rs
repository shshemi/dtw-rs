use std::ops::{Add, Index};

use crate::{Solution, matrix::Matrix};

pub struct DtwSolution<D> {
    mat: Matrix<D>,
}

impl<D> Solution<D> for DtwSolution<D> {
    fn distance(&self) -> D {
        todo!()
    }

    fn path(&self) -> Vec<(usize, usize)> {
        todo!()
    }
}

pub fn dtw_with_distance<T, D>(x: &[T], y: &[T], distance: impl Fn(&T, &T) -> D) -> DtwSolution<D>
where
    D: Ord + Add<Output = D> + Default + Clone,
{
    //
    let mut mat = Matrix::fill(Default::default(), x.len(), y.len());
    for i in 0..x.len() {
        for j in 0..y.len() {
            //
            match (i, j) {
                (0, 0) => (),
                (0, _) | (_, 0) => {
                    mat[(i, j)] = distance(&x[i], &y[i]);
                }
                (_, _) => {
                    let d = distance(&x[i], &y[i]);
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

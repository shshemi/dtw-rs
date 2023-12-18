use std::{cmp::Ordering, fmt::Display, iter::from_fn, ops::Add, usize};

use super::utils::Matrix;
use crate::{Algorithm, ParameterizedAlgorithm};

#[derive(Debug, PartialEq, Clone)]
/// Dynamic time warping computation using the standard dynamic programming method.
pub struct DynamicTimeWarping<D> {
    matrix: Matrix<Element<D>>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Element<T> {
    #[default]
    Inf,
    Value(T),
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum Restriction {
    #[default]
    None,
    Band(usize),
}

impl<D: PartialOrd + Clone + Default + Add<D, Output = D>> Algorithm<D> for DynamicTimeWarping<D> {
    fn with_closure<T>(a: &[T], b: &[T], distance: impl Fn(&T, &T) -> D) -> Self {
        DynamicTimeWarping::with_closure_and_param(a, b, distance, Restriction::None)
    }

    fn distance(&self) -> D {
        let shape = self.matrix.shape();
        match &self.matrix[(shape.0 - 1, shape.1 - 1)] {
            Element::Inf => panic!("Infinit distance"),
            Element::Value(v) => v.clone(),
        }
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
        let mut mat = Matrix::fill(Element::Inf, a.len(), b.len());
        optimize_matrix(&mut mat, hyper_parameters, |i, j| distance(&a[i], &b[j]));
        Self { matrix: mat }
    }
}

impl<D: Display> Display for DynamicTimeWarping<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Dynamic programming computation matrix:\n{}",
            self.matrix
        )
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
        compute_path(&self.matrix, i, j, Restriction::None)
    }
}

impl Restriction {
    pub fn contains(&self, index: (usize, usize), shape: (usize, usize)) -> bool {
        let (rb, re) = self.range(shape, index.0);
        match self {
            Restriction::None => rb <= index.1 && index.1 < re,
            Restriction::Band(_) => rb <= index.1 && index.1 < re,
        }
    }

    pub fn iter(&self, shape: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let restriction = *self;
        let mut idx = None;
        from_fn(move || -> Option<(usize, usize)> {
            if let Some((i, j)) = idx {
                match restriction {
                    Restriction::None => {
                        if i == shape.0 - 1 && j == shape.1 - 1 {
                            idx = None
                        } else if j == shape.1 - 1 {
                            idx = Some((i + 1, 0));
                        } else {
                            idx = Some((i, j + 1));
                        }
                    }
                    Restriction::Band(_) => {
                        let (rb, re) = restriction.range(shape, i);
                        if i == shape.0 - 1 && j == shape.1 - 1 {
                            idx = None
                        } else if j == re {
                            idx = Some((i + 1, rb + 1));
                        } else {
                            idx = Some((i, j + 1));
                        }
                    }
                };
            } else {
                idx = Some((0_usize, 0_usize));
            }
            idx
        })
        .fuse()
    }

    fn range(self, shape: (usize, usize), i: usize) -> (usize, usize) {
        match self {
            Restriction::None => (0, shape.1),
            Restriction::Band(size) => {
                let n1 = shape.0 as f64;
                let n2 = shape.1 as f64;
                let i = i as f64;
                let size = size as f64;
                (
                    (f64::floor(i * (n2 - 1_f64) / (n1 - 1_f64)) - size) as usize,
                    (f64::ceil(i * (n2 - 1_f64) / (n1 - 1_f64)) + size) as usize,
                )
            }
        }
    }
}

impl<T> PartialOrd for Element<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Element::Inf, Element::Inf) => None,
            (Element::Inf, Element::Value(_)) => Some(Ordering::Greater),
            (Element::Value(_), Element::Inf) => Some(Ordering::Less),
            (Element::Value(v1), Element::Value(v2)) => v1.partial_cmp(v2),
        }
    }
}

impl<T> Add for Element<T>
where
    T: Add<Output = T>,
{
    type Output = Element<T>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Element::Inf, _) => Element::Inf,
            (_, Element::Inf) => Element::Inf,
            (Element::Value(v1), Element::Value(v2)) => Element::Value(v1 + v2),
        }
    }
}

impl<T> Display for Element<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Inf => write!(f, "{}", char::from_u32(0xe255).unwrap()),
            Element::Value(v) => write!(f, "{}", v),
        }
    }
}

fn optimize_matrix<D: Clone + PartialOrd + Add<D, Output = D>>(
    matrix: &mut Matrix<Element<D>>,
    restriction: Restriction,
    distance: impl Fn(usize, usize) -> D,
) {
    restriction.iter(matrix.shape()).for_each(|(i, j)| {
        matrix[(i, j)] = preceeding_cost(matrix, (i, j), restriction)
            .map(|idx| matrix[idx].clone() + Element::Value(distance(i, j)))
            .unwrap_or_else(|| Element::Value(distance(i, j)));
    });
}

fn compute_path<D>(
    matrix: &Matrix<Element<D>>,
    i: usize,
    j: usize,
    restriction: Restriction,
) -> Vec<(usize, usize)>
where
    D: PartialOrd,
{
    let mut i = i;
    let mut j = j;
    let mut v = vec![(i, j)];
    while i != 0 || j != 0 {
        if let Some((i_, j_)) = preceeding_cost(matrix, (i, j), restriction) {
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

fn preceeding_cost<D: PartialOrd>(
    matrix: &Matrix<D>,
    index: (usize, usize),
    restriction: Restriction,
) -> Option<(usize, usize)> {
    if restriction.contains(index, matrix.shape()) {
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
    } else {
        None
    }
}

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
    use crate::{algorithms::{
        dynamic_programming::{optimize_matrix, Element},
        utils::Matrix,
    }, Restriction};

    use super::{compute_path, DynamicTimeWarping};

    #[test]
    fn compute_matrix_with_example() {
        let a = [1.0, 3.0, 9.0, 2.0, 1.0];
        let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
        let expected_matrix = Matrix::from_iter(
            vec![
                1.0, 2.0, 3.0, 10.0, 16.0, 17.0, 2.0, 4.0, 5.0, 8.0, 12.0, 13.0, 9.0, 11.0, 13.0,
                6.0, 8.0, 15.0, 9.0, 11.0, 13.0, 12.0, 11.0, 8.0, 10.0, 10.0, 11.0, 18.0, 17.0,
                9.0,
            ]
            .into_iter()
            .map(Element::Value),
            5,
            6,
        );

        let mut matrix = Matrix::fill(Element::Inf, a.len(), b.len());
        optimize_matrix(&mut matrix, crate::Restriction::None, |i, j| {
            f64::abs(a[i] - b[j])
        });
        println!("Matrix:");
        println!("{}", matrix);
        assert!(matrix == expected_matrix);
    }

    #[test]
    fn compute_matrix_restricted_band_with_example() {
        let a = [0.0; 5];
        let b = [0.0; 5];
        let expected_matrix = Matrix::from_iter(
            vec![
                Element::Value(0.0),
                Element::Value(0.0),
                Element::Inf,
                Element::Inf,
                Element::Inf,
                Element::Inf,
                Element::Value(0.0),
                Element::Value(0.0),
                Element::Inf,
                Element::Inf,
                Element::Inf,
                Element::Value(0.0),
                Element::Value(0.0),
                Element::Value(0.0),
                Element::Inf,
                Element::Inf,
                Element::Inf,
                Element::Value(0.0),
                Element::Value(0.0),
                Element::Value(0.0),
                Element::Inf,
                Element::Inf,
                Element::Inf,
                Element::Value(0.0),
                Element::Value(0.0),
            ]
            .into_iter(),
            5,
            5,
        );

        let mut mat = Matrix::fill(Element::Inf, a.len(), b.len());
        optimize_matrix(&mut mat, crate::Restriction::Band(1), |i, j| {
            f64::abs(a[i] - b[j])
        });
        // println!("{}", dtw.matrix);
        // println!("{:?}", dtw.matrix.data().iter().zip(expected_matrix.data().iter()).map(|(e1, e2)| e1 == e2).collect::<Vec<bool>>());
        println!("Matrix:");
        println!("{}", mat);
        println!("Expectation:");
        println!("{}", expected_matrix);
        // assert!(mat == expected_matrix);
        for (e1, e2) in mat.data().iter().zip(expected_matrix.data().iter()) {
            assert_eq!(e1, e2)
        }
    }

    #[test]
    fn compute_path_with_example() {
        let matrix = Matrix::from_iter(
            vec![
                1.0, 2.0, 3.0, 10.0, 16.0, 17.0, 2.0, 4.0, 5.0, 8.0, 12.0, 13.0, 9.0, 11.0, 13.0,
                6.0, 8.0, 15.0, 9.0, 11.0, 13.0, 12.0, 11.0, 8.0, 10.0, 10.0, 11.0, 18.0, 17.0,
                9.0,
            ]
            .into_iter()
            .map(Element::Value),
            5,
            6,
        );
        let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
        let founded_path = compute_path(&matrix, 4, 5, crate::Restriction::None);
        assert!(expected_path == *founded_path);
    }

    #[test]
    fn partial_ord_element() {
        assert!(Element::Value(-1) < Element::Value(0));
        assert!(Element::Value(0) < Element::Value(1));
        assert!(Element::Value(1) < Element::Inf);
    }

    #[test]
    fn iter_contain_restriction() {
        let shape = (5_usize, 6_usize);
        let no_rest = Restriction::None;
        let restriction = Restriction::Band(1);
        let all_indices = no_rest.iter(shape).collect::<Vec<(usize, usize)>>();
        let band_indices = restriction.iter(shape).collect::<Vec<(usize, usize)>>();
        for idx in all_indices.into_iter(){
            assert_eq!(band_indices.contains(&idx), restriction.contains(idx, shape));
        }
        
    }

    fn sized_send_sync_unpin_check<T: Sized + Send + Sync + Unpin>() {}
    #[test]
    fn check_auto_traits() {
        sized_send_sync_unpin_check::<DynamicTimeWarping<f64>>()
    }
}

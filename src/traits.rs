use std::ops::Sub;

/// Compute the dynamic time warping of two sequence.
pub trait Algorithm {
    /// Warped distance between the input sequences as a `f64`.
    fn distance(&self) -> f64;

    /// Warped path between the input sequences as a `Vec<(usize, usize)>`.
    fn path(&self) -> Vec<(usize, usize)>;

    /// Dynamic time warping between sequences `a` and `b` according to the distance closure
    /// `distance`.
    fn with_closure<T>(a: &[T], b: &[T], distance: impl Fn(&T, &T) -> f64) -> Self;

    /// Dynamic time warping between sequences `a: &[T]` and `b: &[T]` according the implemented
    /// `Distance` trait.
    fn between<T>(a: &[T], b: &[T]) -> Self
    where
        T: Distance,
        Self: Sized,
    {
        Self::with_closure(a, b, |a, b| a.distance(b))
    }
}

/// Compute the dynamic time warping of two sequence with initial hyper-parameters.
pub trait ParameterizedAlgorithm {
    type Param;

    /// Dynamic time warping between sequences `a` and `b` according to the distance closure
    /// `distance` and parameters `param`.
    fn with_closure_and_hyper_parameters<T>(
        a: &[T],
        b: &[T],
        distance: impl Fn(&T, &T) -> f64,
        param: Self::Param,
    ) -> Self;

    /// Dynamic time warping between sequences `a` and `b` according the implemented `Distance` trait
    /// and hyper-parameters `param`
    fn with_hyper_parameters<T>(a: &[T], b: &[T], param: Self::Param) -> Self
    where
        T: Distance,
        Self: Sized,
    {
        Self::with_closure_and_hyper_parameters(a, b, |a, b| a.distance(b), param)
    }
}

/// The distance between two type.
pub trait Distance {
    /// Distance between `self` and `other` as a `f64`.
    fn distance(&self, other: &Self) -> f64;
}

/// Blanket implementation for primitive numerical types such as f32, f64, i32, and ...
impl<T, O> Distance for T
where
    O: Into<f64>,
    T: Sub<Output = O> + PartialOrd + Copy,
    Self: Sized,
{
    fn distance(&self, other: &Self) -> f64 {
        if self > other {
            *self - *other
        } else {
            *other - *self
        }
        .into()
    }
}

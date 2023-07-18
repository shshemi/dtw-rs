use std::ops::Sub;

/// Compute the dynamic time warping of two sequence.
pub trait Algorithm {
    /// Warped distance between `a` and `b`.
    fn distance(&self) -> f64;

    /// Warped path between `a` and `b`.
    fn path(&self) -> Vec<(usize, usize)>;

    /// Dynamic time warping between sequences `a` and `b` using the distance closure `distance`.
    fn with_closure<T>(a: &[T], b: &[T], distance: impl Fn(&T, &T) -> f64) -> Self;

    /// Dynamic time warping between sequences `a` and `b`
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

    /// Dynamic time warping between sequences `a` and `b` using the distance closure `distance`
    /// and parameter `param`.
    fn with_closure_and_param<T>(
        a: &[T],
        b: &[T],
        distance: impl Fn(&T, &T) -> f64,
        param: Self::Param,
    ) -> Self;

    /// Dynamic time warping between sequences `a` and `b` using the parameter `param`
    fn with_param<T>(a: &[T], b: &[T], param: Self::Param) -> Self
    where
        T: Distance,
        Self: Sized,
    {
        Self::with_closure_and_param(a, b, |a, b| a.distance(b), param)
    }
}

/// An arbitrary distance between two objects.
pub trait Distance {
    /// Distance between `self` and `other`.
    fn distance(&self, other: &Self) -> f64;
}

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

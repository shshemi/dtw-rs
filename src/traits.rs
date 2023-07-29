use std::ops::Sub;

/// Compute the dynamic time warping of two sequence.
pub trait Algorithm<O> {
    /// Warped distance between `a` and `b`.
    fn distance(&self) -> O;

    /// Warped path between `a` and `b`.
    fn path(&self) -> Vec<(usize, usize)>;

    /// Dynamic time warping between sequences `a` and `b` using the distance closure `distance`.
    fn with_closure<T>(a: &[T], b: &[T], distance: impl Fn(&T, &T) -> O) -> Self;

    /// Dynamic time warping between sequences `a` and `b`
    fn between<T>(a: &[T], b: &[T]) -> Self
    where
        T: Distance<O>,
        Self: Sized,
    {
        Self::with_closure(a, b, |a, b| a.distance(b))
    }
}

/// Compute the dynamic time warping of two sequence with initial hyper-parameters.
pub trait ParameterizedAlgorithm<D> {
    type Param;

    /// Dynamic time warping between sequences `a` and `b` using the distance closure `distance`
    /// and parameter `param`.
    fn with_closure_and_param<T>(
        a: &[T],
        b: &[T],
        distance: impl Fn(&T, &T) -> D,
        param: Self::Param,
    ) -> Self;

    /// Dynamic time warping between sequences `a` and `b` using the parameter `param`
    fn with_param<T>(a: &[T], b: &[T], param: Self::Param) -> Self
    where
        T: Distance<D>,
        Self: Sized,
    {
        Self::with_closure_and_param(a, b, |a, b| a.distance(b), param)
    }
}

/// An arbitrary distance between two objects.
pub trait Distance<O>
{
    /// Distance between `self` and `other`.
    fn distance(&self, other: &Self) -> O;
}

impl<T, O> Distance<O> for T
where
    O: PartialOrd,
    T: Sub<Output = O> + PartialOrd + Copy,
    Self: Sized,
{
    fn distance(&self, other: &Self) -> O {
        if self > other {
            *self - *other
        } else {
            *other - *self
        }
        // .into()
    }

}

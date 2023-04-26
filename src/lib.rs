//! # DTW_RS
//! This crate implements multiple algorithm, including Dynamic Programming and FastDTW, to compute dyanmic
//! time warping between two sequence.
//!
//!

/// The module contains different implementaion of dynamic time warping.
pub mod methods;

use std::ops::Sub;

/// The trait is implemented by types which offer a calculation or estimation of the dynamic time warping problem.
pub trait BasicMethod {
    /// Return the warped distance between the input sequences as a `f64`.
    fn distance(&self) -> f64;

    /// Return the warped path between the input sequences as a `Vec<(usize, usize)>`.
    fn path(&self) -> Vec<(usize, usize)>;

    /// Calculate the dynamic time warping between sequences `a` and `b` according to the distance
    /// closure `distance`.
    fn with_closure<T>(a: &[T], b: &[T], distance: impl Fn(&T, &T) -> f64) -> Self;

    /// Calculate the dynamic time warping between sequences `a: &[T]` and `b: &[T]` according the
    /// implemented `Distance` trait.
    fn between<T>(a: &[T], b: &[T]) -> Self
    where
        T: Distance,
        Self: Sized,
    {
        Self::with_closure(a, b, |a, b| a.distance(b))
    }

    /// Calculate the dynamic time warping between sequences `a` and `b` while using the absolute
    /// distance as the distance metric.
    fn with_absolute_distance<T, O>(a: &[T], b: &[T]) -> Self
    where
        O: Into<f64>,
        T: Sub<Output = O> + PartialOrd + Copy,
        Self: Sized,
    {
        Self::with_closure(a, b, |a: &T, b: &T| {
            if a > b { *a - *b } else { *b - *a }.into()
        })
    }
}

/// The trait is implemented by types which offer a more sophisticated calculation or estimation, which requires
/// initial hyper-parameters, of the dynamic time warping problem.
pub trait ParameterizedMethod: BasicMethod {
    type Parameters;

    /// Calculate the dynamic time warping between sequences `a` and `b` according to the distance
    /// closure `distance` and hyper-parameters `hyper_parameters`.
    fn with_closure_and_hyper_parameters<T>(
        a: &[T],
        b: &[T],
        distance: impl Fn(&T, &T) -> f64,
        hyper_parameters: Self::Parameters,
    ) -> Self;

    /// Calculate the dynamic time warping between sequences `a` and `b` according to the distance
    /// closure `distance` with default hyper-parameters.
    fn with_closure<T>(a: &[T], b: &[T], distance: impl Fn(&T, &T) -> f64) -> Self
    where
        Self::Parameters: Default,
        Self: Sized,
    {
        Self::with_closure_and_hyper_parameters(a, b, distance, Self::Parameters::default())
    }

    /// Calculate the dynamic time warping between sequences `a` and `b` according the implemented
    /// `Distance` trait and hyper-parameters `hyper_parameters`
    fn with_hyper_parameters<T>(a: &[T], b: &[T], hyper_parameters: Self::Parameters) -> Self
    where
        T: Distance,
        Self: Sized,
    {
        Self::with_closure_and_hyper_parameters(a, b, |a, b| a.distance(b), hyper_parameters)
    }
}
/// The trait should be implemented for custom types which are intented to be useded as the input for dynamic time
/// warping without any distance closure.
pub trait Distance {
    /// Return the distance between `self` and `other` as a `f64`.
    fn distance(&self, other: &Self) -> f64;
}

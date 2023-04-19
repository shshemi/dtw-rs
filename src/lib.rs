//! # DTW_RS
//! This crate implements multiple algorithm, including Dynamic Programming and FastDTW, to compute dyanmic
//! time warping between two sequence.
//!
//!

/// The module containing the the classic dynamic programming implementaion of dynamic time warping.
/// The content itself should not be initialized directly, and the implementation should be utilzed with the builder class.
pub mod dynamic_programming;

use std::default::Default;
use std::{marker::PhantomData, ops::Sub};

/// The struct for builder pattern which enables dynamic time warping computation. The builder should
/// be created via `DynamicTimeWarping::default()`. At this state, the distance between two elements
/// using the `Distance` trait.
pub struct DynamicTimeWarping<A: DyanmicTimeWarpingAlgorithm, S = NoCallback> {
    a: PhantomData<A>,
    s: S,
}

#[derive(Default)]
pub struct NoCallback;
pub struct WithCallback<T, C>(C, PhantomData<T>)
where
    C: Fn(&T, &T) -> f64;

/// The struct is the same as `DynamicTimeWarping` except that it has a `DistanceClosure` which will be
/// used to measure the distance between two elements.
// pub struct DynamicTimeWarpigWithDistanceClosure<A, T> {
//     a: PhantomData<A>,
//     distance: DistanceClosure<T>,
// }

/// The trait is used to define the behavior of algorithms that compute the dynamic time warping between
/// two sequences. Any type which intent to operate with `DynamicTimeWarping` should implement `between`
/// and `between_closure` assosiated functions and `distance` and `path` methods.
pub trait DyanmicTimeWarpingAlgorithm {
    fn between<T: Distance>(a: &[T], b: &[T]) -> Self;
    fn between_closure<T>(a: &[T], b: &[T], distance: impl Fn(&T, &T) -> f64) -> Self;
    fn distance(&self) -> f64;
    fn path(&self) -> Vec<(usize, usize)>;
}

/// The `Distance` trait defines a method `distance` which takes a reference to another object of the
/// same type and returns a `f64` value representing the distance between the two objects. This trait is
/// used to measure the distance between two elements in the sequence being compared in the dynamic time
/// warping algorithm.
pub trait Distance {
    fn distance(&self, other: &Self) -> f64;
}

impl<A: DyanmicTimeWarpingAlgorithm> Default for DynamicTimeWarping<A, NoCallback> {
    fn default() -> Self {
        Self {
            a: Default::default(),
            s: Default::default(),
        }
    }
}

impl<A: DyanmicTimeWarpingAlgorithm> DynamicTimeWarping<A, NoCallback> {
    /// This function returns a new instance of DynamicTimeWarping with a custom distance closure.
    ///
    /// Arguments:
    ///
    /// * `distance`: `distance` is a distance closure that takes two arguments of type `&T` and returns a `f64`
    /// which represents the distance between thw two elements
    ///
    /// Returns:
    ///
    /// The function `with_custom_distance` returns a new instance of `DynamicTimeWarping` with the same `a`
    /// value as the original instance, but with a new `s` value that is a `WithCallback` struct containing
    /// the `distance` closure passed as an argument.
    pub fn with_custom_distance<T>(
        self,
        distance: impl Fn(&T, &T) -> f64,
    ) -> DynamicTimeWarping<A, WithCallback<T, impl Fn(&T, &T) -> f64>> {
        DynamicTimeWarping {
            a: self.a,
            s: WithCallback(distance, Default::default()),
        }
    }

    /// The function absolute distance closure to make the computation of dynamic time warping possible for
    /// types that do not implement Distance trait
    ///
    /// Returns:
    ///
    /// `DynamicTimeWarping` builder with absolute distance
    pub fn with_absolute_distance<T, O>(
        self,
    ) -> DynamicTimeWarping<A, WithCallback<T, impl Fn(&T, &T) -> f64>>
    where
        O: Into<f64>,
        T: Sub<Output = O> + PartialOrd + Copy,
    {
        self.with_custom_distance(|a:&T, b:&T| { if a > b { *a - *b } else { *b - *a }.into() })
    }

    /// The function accepts two slide of type T and compute the dynamic warping distance with respect to the
    /// distance trait implemented by type T.
    ///
    /// Arguments:
    ///
    /// * `a`: `a` is a slice of type `&[T:Distance]` that represents the 1st sequence.
    /// * `b`: `b` is a slice of type `&[T:Distance]` that represents the 2nd sequence.
    ///
    /// Returns:
    ///
    /// The dynaimc warpped path and the distance between a and b.
    pub fn compute<T: Distance>(&self, a: &[T], b: &[T]) -> A {
        A::between(a, b)
    }
}

impl<A: DyanmicTimeWarpingAlgorithm, T, C: Fn(&T, &T) -> f64>
    DynamicTimeWarping<A, WithCallback<T, C>>
{
    /// The function accepts two slide of type T and compute the dynamic warping distance with respect to the
    /// distance closure block what was passed.
    ///
    /// Arguments:
    ///
    /// * `a`: `a` is a slice of type `&[T]` that represents the 1st sequence.
    /// * `b`: `b` is a slice of type `&[T]` that represents the 2nd sequence.
    ///
    /// Returns:
    ///
    /// The dynaimc warpped path and the distance between a and b.
    pub fn compute(&self, a: &[T], b: &[T]) -> A {
        A::between_closure(a, b, &self.s.0)
    }
}

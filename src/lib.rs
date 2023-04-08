pub mod dynamic_programming;

use std::default::Default;
use std::{marker::PhantomData, ops::Sub};

pub trait Distance {
    fn distance(&self, other: &Self) -> f64;
}

pub type DistanceClosure<T> = Box<dyn Fn(&T, &T) -> f64>;

pub trait DyanmicTimeWarpingAlgorithm {
    fn between<T: Distance>(a: &[T], b: &[T]) -> Self;
    fn between_closure<T>(a: &[T], b: &[T], distance: impl Fn(&T, &T) -> f64) -> Self;
    fn distance(&self) -> f64;
    fn path(&self) -> Vec<(usize, usize)>;
}

pub struct DynamicTimeWarping<A: DyanmicTimeWarpingAlgorithm> {
    a: PhantomData<A>,
}
pub struct DynamicTimeWarpigWithDistanceClosure<A, T> {
    a: PhantomData<A>,
    distance: DistanceClosure<T>,
}

impl<A: DyanmicTimeWarpingAlgorithm> Default for DynamicTimeWarping<A> {
    fn default() -> Self {
        Self {
            a: Default::default(),
        }
    }
}

impl<A> DynamicTimeWarping<A>
where
    A: DyanmicTimeWarpingAlgorithm,
{
    pub fn compute<T: Distance>(&self, a: &[T], b: &[T]) -> A {
        A::between(a, b)
    }

    pub fn with_custom_distance<T>(
        distance: DistanceClosure<T>,
    ) -> DynamicTimeWarpigWithDistanceClosure<A, T> {
        DynamicTimeWarpigWithDistanceClosure {
            a: Default::default(),
            distance,
        }
    }

    pub fn with_absolute_distance<O, T>(self) -> DynamicTimeWarpigWithDistanceClosure<A, T>
    where
        O: Into<f64>,
        T: Sub<Output = O> + PartialOrd + Copy,
    {
        DynamicTimeWarpigWithDistanceClosure {
            a: self.a,
            distance: Box::new(|a, b| if a > b { *a - *b } else { *b - *a }.into()),
        }
    }
}

impl<A, T> DynamicTimeWarpigWithDistanceClosure<A, T>
where
    A: DyanmicTimeWarpingAlgorithm,
{
    pub fn compute(&self, a: &[T], b: &[T]) -> A {
        A::between_closure(a, b, &self.distance)
    }
}

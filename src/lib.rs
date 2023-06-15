//! # DTW_RS
//! This crate implements multiple algorithm, including Dynamic Programming and FastDTW, to compute dyanmic
//! time warping between two sequence.
//!
//!

/// The module contains different implementaion of dynamic time warping.
mod algorithms;
mod traits;
pub use traits::{Distance, Algorithm, ParameterizedAlgorithm};
pub use algorithms::{DynamicTimeWarping, Restriction};
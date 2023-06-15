//! # DTW_RS
//! This crate implements multiple algorithm, including Dynamic Programming and FastDTW, to compute dyanmic
//! time warping between two sequence.
//!
//!

/// The module contains different implementaion of dynamic time warping.
pub mod methods;
mod traits;
pub use traits::{Distance, DynamicTimeWarping, ParameterizedDynamicTimeWarping};
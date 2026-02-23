//! A zero-dependency Dynamic Time Warping (DTW) library for Rust.
//!
//! Dynamic Time Warping measures the similarity between two temporal sequences
//! that may vary in speed. It finds the optimal alignment (warping path) between
//! sequences by minimizing the total distance, allowing elements to be matched
//! non-linearly.
//!
//! # Algorithms
//!
//! This crate provides four DTW algorithms:
//!
//! - **[`dtw`]** — Standard DTW using dynamic programming. Exact solution with
//!   O(n*m) time and space complexity. Best for short sequences or when an exact
//!   result is required.
//! - **[`sakoe_chiba`]** — DTW constrained by a Sakoe-Chiba band. Restricts the
//!   warping path to a band around the diagonal, reducing computation while still
//!   producing a good alignment.
//! - **[`itakura_parallelogram`]** — DTW constrained by an Itakura parallelogram.
//!   Limits the warping path to a parallelogram-shaped region, preventing excessive
//!   compression or stretching.
//! - **[`fastdtw`]** — An approximate DTW algorithm that recursively coarsens the
//!   sequences and projects the warping path, achieving roughly O(n) time.
//!
//! Each algorithm has a `_with_distance` variant (e.g., [`dtw_with_distance`]) that
//! accepts a custom distance closure instead of relying on the [`Distance`] trait.
//!
//! # Quick Start
//!
//! ```
//! use dtw_rs::{dtw, sakoe_chiba, itakura_parallelogram, fastdtw, Solution};
//!
//! let a = [1.0_f64, 3.0, 9.0, 2.0, 1.0];
//! let b = [2.0_f64, 0.0, 0.0, 8.0, 7.0, 2.0];
//!
//! // Unconstrained DTW
//! let result = dtw(&a, &b);
//! println!("Distance: {}, Path: {:?}", result.distance(), result.path());
//!
//! // Sakoe-Chiba band constraint
//! let result = sakoe_chiba(&a, &b, 1);
//! println!("Distance: {}, Path: {:?}", result.distance(), result.path());
//!
//! // Itakura parallelogram constraint
//! let result = itakura_parallelogram(&a, &b, 2.0);
//! println!("Distance: {}, Path: {:?}", result.distance(), result.path());
//!
//! // FastDTW (approximate, with radius parameter)
//! let result = fastdtw(&a, &b, 1);
//! println!("Distance: {}, Path: {:?}", result.distance(), result.path());
//! ```
//!
//! # Custom Distance Functions
//!
//! Use the `_with_distance` variants to supply your own distance metric:
//!
//! ```
//! use dtw_rs::{dtw_with_distance, Solution};
//!
//! let a = [1.0, 3.0, 9.0, 2.0, 1.0];
//! let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
//!
//! let result = dtw_with_distance(&a, &b, |a: &f64, b: &f64| (a - b).powi(2));
//! println!("Squared distance: {}", result.distance());
//! ```

mod matrix;
mod methods;
mod traits;
pub use methods::dynamic_programming::{dtw, dtw_with_distance};
pub use methods::fast_dtw::{fastdtw, fastdtw_with_distance};
pub use methods::itakura_parallelogram::{
    itakura_parallelogram, itakura_parallelogram_with_distance,
};
pub use methods::sakoe_chiba::{sakoe_chiba, sakoe_chiba_with_distance};
pub use traits::{Distance, Midpoint, Solution};

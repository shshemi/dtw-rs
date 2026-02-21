mod matrix;
mod methods;
mod traits;
pub use methods::dynamic_programming::{dtw, dtw_with_distance};
pub use methods::itakura_parallelogram::{itakura_parallelogram, itakura_parallelogram_with_distance};
pub use traits::{Distance, Solution};

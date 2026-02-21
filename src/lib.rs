mod matrix;
mod methods;
mod traits;
pub use methods::dynamic_programming::{dtw, dtw_with_distance};
pub use traits::{Distance, Solution};

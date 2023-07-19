/*!
# DTW_RS
A Dynamic Time Warping (DTW) library for Rust

Computation methods:
- [x] Dynamic programming
- [x] Dynamic programming with the Sakoe-Chuba Band
- [ ] Dynamic programming with the Itakura Parallelogram (future plan)
- [ ] FastDTW (future plan)

```
use dtw_rs::{Algorithm, DynamicTimeWarping};

let a = [1.0, 3.0, 9.0, 2.0, 1.0];
let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];

let dtw = DynamicTimeWarping::between(&a, &b);

println!("Distance: {}, Path: {:?}", dtw.distance(), dtw.path());

```

*/

mod algorithms;
mod traits;
pub use algorithms::{DynamicTimeWarping, Restriction};
pub use traits::{Algorithm, Distance, ParameterizedAlgorithm};

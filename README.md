# DTW_RS
A Dynamic Time Warping (DTW) library for Rust

Computation methods:
- [x] Dynamic programming
- [x] Dynamic programming with the Sakoe-Chuba Band
- [ ] Dynamic programming with the Itakura Parallelogram (future plan)
- [ ] FastDTW (future plan)

Install:
```bash
cargo add dtw_rs
```
Usage: 
```rust
use dtw_rs::{Algorithm, DynamicTimeWarping};

let a = [1.0, 3.0, 9.0, 2.0, 1.0];
let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];

let dtw = DynamicTimeWarping::with_closure(&a, &b, |a, b| f64::abs(a - b));

println!("Distance: {}, Path: {:?}", dtw.distance(), dtw.path());

```
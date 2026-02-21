# DTW_RS
A Dynamic Time Warping (DTW) library for Rust

Computation methods:
- [x] Dynamic programming
- [x] Dynamic programming with the Sakoe-Chiba Band
- [x] Dynamic programming with the Itakura Parallelogram
- [ ] FastDTW (future plan)

Install:
```bash
cargo add dtw_rs
```

Usage:
```rust
use dtw_rs::{dtw, sakoe_chiba, itakura_parallelogram, Solution};

let a = [1.0_f64, 3.0, 9.0, 2.0, 1.0];
let b = [2.0_f64, 0.0, 0.0, 8.0, 7.0, 2.0];

// Unconstrained DTW
let result = dtw(&a, &b);
println!("Distance: {}, Path: {:?}", result.distance(), result.path());

// Sakoe-Chiba band constraint
let result = sakoe_chiba(&a, &b, 1);
println!("Distance: {}, Path: {:?}", result.distance(), result.path());

// Itakura parallelogram constraint
let result = itakura_parallelogram(&a, &b, 2.0);
println!("Distance: {}, Path: {:?}", result.distance(), result.path());
```

Custom distance functions:
```rust
use dtw_rs::{dtw_with_distance, Solution};

let a = [1.0, 3.0, 9.0, 2.0, 1.0];
let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];

let result = dtw_with_distance(&a, &b, |a, b| (a - b).powi(2));
println!("Distance: {}, Path: {:?}", result.distance(), result.path());
```

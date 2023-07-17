use std::str::FromStr;

use dtw_rs::{Algorithm, Distance, DynamicTimeWarping, ParameterizedAlgorithm, Restriction};
use float_cmp::assert_approx_eq;

#[test]
fn dynamic_time_warping_distance_trait() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0].map(MockF64);
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0].map(MockF64);
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
    let expected_distance = 9.0;

    let dtw = DynamicTimeWarping::between(&a, &b);
    println!("Matrix:");
    println!("{}", dtw);
    println!("Path: {:?}", dtw.path());
    assert!(dtw.distance() == expected_distance);
    assert!(*dtw.path() == expected_path);
}

#[test]
fn dynamic_time_warping_absolute_distance() {
    let const_str = include_str!("dtw_test_cases.txt");
    const_str
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(5)
        .for_each(|chunk| {
            let a = into_float_vec::<f64>(chunk[0]);
            let b = into_float_vec::<f64>(chunk[1]);
            let expected_path = into_float_vec::<usize>(chunk[2])
                .into_iter()
                .zip(into_float_vec::<usize>(chunk[3]))
                .collect::<Vec<(usize, usize)>>();
            let expected_distance = chunk[4].parse::<f64>().unwrap();

            let dtw = DynamicTimeWarping::with_absolute_distance(&a, &b);
            assert_approx_eq!(f64, dtw.distance(), expected_distance);
            assert_eq!(*dtw.path(), expected_path);
        });
}

#[test]
fn dynamic_time_warping_custom_distance() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0];
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
    let expected_distance = 9.0;

    let dtw = DynamicTimeWarping::with_closure(&a, &b, |a, b| f64::abs(a - b));
    println!("Matrix:");
    println!("{}", dtw);
    println!("Path: {:?}", dtw.path());
    assert!(dtw.distance() == expected_distance);
    assert!(*dtw.path() == expected_path);
}

#[test]
fn dynamic_time_warping_band_restricted_distance_trait() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0].map(MockF64);
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0].map(MockF64);
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (3, 4), (4, 5)];
    let expected_distance = 12.0;

    let dtw = DynamicTimeWarping::with_hyper_parameters(&a, &b, Restriction::Band(1));
    println!("Matrix:");
    println!("{}", dtw);
    println!("Path: {:?}", dtw.path());
    assert!(dtw.distance() == expected_distance);
    assert!(*dtw.path() == expected_path);
}

#[test]
fn dynamic_time_warping_band_restricted_custom_distance() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0];
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (3, 4), (4, 5)];
    let expected_distance = 12.0;

    let dtw = DynamicTimeWarping::with_closure_and_hyper_parameters(
        &a,
        &b,
        |a, b| f64::abs(a - b),
        Restriction::Band(1),
    );
    println!("Matrix:");
    println!("{}", dtw);
    println!("Path: {:?}", dtw.path());
    assert!(dtw.distance() == expected_distance);
    assert!(*dtw.path() == expected_path);
}

struct MockF64(f64);

impl Distance for MockF64 {
    fn distance(&self, other: &Self) -> f64 {
        f64::abs(self.0 - other.0)
    }
}

#[inline]
fn into_float_vec<T: FromStr>(line: &str) -> Vec<T> {
    line.split(' ')
        .map(str::parse::<T>)
        .filter_map(Result::ok)
        .collect::<Vec<T>>()
}

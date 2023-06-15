use dtw_rs::{
    DynamicTimeWarping, Restriction, Distance, Algorithm,
    ParameterizedAlgorithm,
};

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
    let a = [1.0, 3.0, 9.0, 2.0, 1.0];
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
    let expected_distance = 9.0;

    let dtw = DynamicTimeWarping::with_absolute_distance(&a, &b);
    println!("Matrix:");
    println!("{}", dtw);
    println!("Path: {:?}", dtw.path());
    assert!(dtw.distance() == expected_distance);
    assert!(*dtw.path() == expected_path);
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

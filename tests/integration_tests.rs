use dtw_rs::{methods::DynamicProgramming, BasicMethod, Distance};

#[test]
fn dynamic_programming_distance_trait() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0].map(MockF64);
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0].map(MockF64);
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
    let expected_distance = 9.0;

    let dtw = DynamicProgramming::between(&a, &b);
    println!("Matrix:");
    println!("{}", dtw);
    println!("Path: {:?}", dtw.path());
    assert!(dtw.distance() == expected_distance);
    assert!(*dtw.path() == expected_path);
}

#[test]
fn dynamic_programming_absolute_distance() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0];
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
    let expected_distance = 9.0;

    let dtw = DynamicProgramming::with_absolute_distance(&a, &b);
    println!("Matrix:");
    println!("{}", dtw);
    println!("Path: {:?}", dtw.path());
    assert!(dtw.distance() == expected_distance);
    assert!(*dtw.path() == expected_path);
}

#[test]
fn dynamic_programming_custom_distance() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0];
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
    let expected_distance = 9.0;

    let dtw = DynamicProgramming::with_closure(&a, &b, |a, b| f64::abs(a - b));
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

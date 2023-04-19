use dtw_rs::{
    dynamic_programming::DynamicProgramming, Distance, DyanmicTimeWarpingAlgorithm,
    DynamicTimeWarping,
};
use std::default::Default;


#[test]
fn end_to_end_f64() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0].map(MockF64);
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0].map(MockF64);
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
    let expected_distance = 9.0;
    let dtw = DynamicTimeWarping::<DynamicProgramming>::default().compute(&a, &b);
    println!("Matrix:");
    println!("{}", dtw);
    println!("Path: {:?}", dtw.path());
    assert!(dtw.distance() == expected_distance);
    assert!(*dtw.path() == expected_path);
}

#[test]
fn end_to_end_char() {
    let a = "abbc".chars().map(MockChar).collect::<Vec<MockChar>>();
    let b = "abc".chars().map(MockChar).collect::<Vec<MockChar>>();
    let dtw = DynamicTimeWarping::<DynamicProgramming>::default().compute(&a, &b);
    println!("Matrix:");
    println!("{}", dtw);
    println!("Path: {:?}", dtw.path());
    assert!(dtw.distance() == 0.0);
    assert!(*dtw.path() == [(0, 0), (1, 1), (2, 1), (3, 2)]);
}

#[test]
fn end_to_end_f64_with_absolute_distance() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0];
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
    let expected_distance = 9.0;

    let dtw = DynamicTimeWarping::<DynamicProgramming>::default()
        .with_absolute_distance()
        .compute(&a, &b);
    println!("Matrix:");
    println!("{}", dtw);
    println!("Path: {:?}", dtw.path());
    assert!(dtw.distance() == expected_distance);
    assert!(*dtw.path() == expected_path);
}

#[test]
fn end_to_end_f64_with_custom_distance() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0];
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
    let expected_distance = 9.0;

    let dtw = DynamicTimeWarping::<DynamicProgramming>::default()
        .with_custom_distance(|a, b|{
            f64::abs(a - b)
        })
        .compute(&a, &b);
    println!("Matrix:");
    println!("{}", dtw);
    println!("Path: {:?}", dtw.path());
    assert!(dtw.distance() == expected_distance);
    assert!(*dtw.path() == expected_path);
}

struct MockF64(f64);
struct MockChar(char);

impl Distance for MockF64 {
    fn distance(&self, other: &Self) -> f64 {
        f64::abs(self.0 - other.0)
    }
}

impl Distance for MockChar {
    fn distance(&self, other: &Self) -> f64 {
        if self.0 == other.0 {
            0.0
        } else {
            1.0
        }
    }
}

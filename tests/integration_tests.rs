use std::str::FromStr;

use dtw_rs::{
    Solution, dtw, dtw_with_distance, fastdtw, fastdtw_with_distance, itakura_parallelogram,
    itakura_parallelogram_with_distance, sakoe_chiba, sakoe_chiba_with_distance,
};
use float_cmp::assert_approx_eq;

#[test]
fn dynamic_time_warping() {
    include_str!("dtw_test_cases.txt")
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

<<<<<<< HEAD
            let dtw = DynamicTimeWarping::between(&a, &b);

            assert_approx_eq!(f64, dtw.distance(), expected_distance);
            assert_eq!(*dtw.path(), expected_path);
        });
}

#[test]
fn dynamic_time_warping_with_distance_closure() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0];
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (2, 4), (3, 5), (4, 5)];
    let expected_distance = 9.0;

    let dtw = DynamicTimeWarping::with_closure(&a, &b, |a, b| f64::abs(a - b));

    assert_eq!(dtw.distance(), expected_distance);
    assert_eq!(*dtw.path(), expected_path);
}

#[test]
fn dynamic_time_warping_with_band_restriction() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0];
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (3, 4), (4, 5)];
    let expected_distance = 12.0;

    let dtw = DynamicTimeWarping::with_param(&a, &b, Restriction::Band(1));

    assert_eq!(dtw.distance(), expected_distance);
    assert_eq!(*dtw.path(), expected_path);
}
#[test]
fn dynamic_time_warping_with_corner_out_of_band() {
    let a = [1.0, 3.0, 9.0, 2.0];
    let b = [2.0, 0.0];
    // The path should actually be cut off at (2, 1)
    let expected_path = [(0, 0), (1, 0), (2, 1), (3, 1)];
    let expected_distance = 11.0;

    let dtw = DynamicTimeWarping::with_param(&a, &b, Restriction::Band(1));

    assert_eq!(dtw.distance(), expected_distance);
    assert_eq!(*dtw.path(), expected_path);
}

#[test]
fn dynamic_time_warping_with_band_restricted_and_distance_closure() {
    let a = [1.0, 3.0, 9.0, 2.0, 1.0];
    let b = [2.0, 0.0, 0.0, 8.0, 7.0, 2.0];
    let expected_path = [(0, 0), (0, 1), (1, 2), (2, 3), (3, 4), (4, 5)];
    let expected_distance = 12.0;

    let dtw = DynamicTimeWarping::with_closure_and_param(
        &a,
        &b,
        |a, b| f64::abs(a - b),
        Restriction::Band(1),
    );

    println!("{}", dtw);
    assert_eq!(dtw.distance(), expected_distance);
    assert_eq!(*dtw.path(), expected_path);
}

=======
            let dtw = dtw_with_distance(&a, &b, |a, b| f64::abs(a - b));

            assert_approx_eq!(f64, dtw.distance(), expected_distance);
            assert_eq!(dtw.path(), expected_path);
        });
}

#[test]
fn dtw_from_test_cases() {
    include_str!("dtw_test_cases.txt")
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

            let result = dtw(&a, &b);

            assert_approx_eq!(f64, result.distance(), expected_distance);
            assert_eq!(result.path(), expected_path);
        });
}

#[test]
fn itakura_parallelogram_with_distance_closure() {
    include_str!("itakura_test_cases.txt")
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

            let result = itakura_parallelogram_with_distance(&a, &b, 2.0, |a, b| f64::abs(a - b));

            assert_approx_eq!(f64, result.distance(), expected_distance);
            assert_eq!(result.path(), expected_path);
        });
}

#[test]
fn itakura_parallelogram_with_distance_trait() {
    include_str!("itakura_test_cases.txt")
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

            let result = itakura_parallelogram(&a, &b, 2.0);

            assert_approx_eq!(f64, result.distance(), expected_distance);
            assert_eq!(result.path(), expected_path);
        });
}

#[test]
fn sakoe_chiba_with_distance_closure() {
    include_str!("sakoe_chiba_test_cases.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(6)
        .for_each(|chunk| {
            let a = into_float_vec::<f64>(chunk[0]);
            let b = into_float_vec::<f64>(chunk[1]);
            let expected_path = into_float_vec::<usize>(chunk[2])
                .into_iter()
                .zip(into_float_vec::<usize>(chunk[3]))
                .collect::<Vec<(usize, usize)>>();
            let expected_distance = chunk[4].parse::<f64>().unwrap();
            let window_size = chunk[5].parse::<usize>().unwrap();

            let result = sakoe_chiba_with_distance(&a, &b, window_size, |a, b| f64::abs(a - b));

            assert_approx_eq!(f64, result.distance(), expected_distance);
            assert_eq!(result.path(), expected_path);
        });
}

#[test]
fn sakoe_chiba_with_distance_trait() {
    include_str!("sakoe_chiba_test_cases.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(6)
        .for_each(|chunk| {
            let a = into_float_vec::<f64>(chunk[0]);
            let b = into_float_vec::<f64>(chunk[1]);
            let expected_path = into_float_vec::<usize>(chunk[2])
                .into_iter()
                .zip(into_float_vec::<usize>(chunk[3]))
                .collect::<Vec<(usize, usize)>>();
            let expected_distance = chunk[4].parse::<f64>().unwrap();
            let window_size = chunk[5].parse::<usize>().unwrap();

            let result = sakoe_chiba(&a, &b, window_size);

            assert_approx_eq!(f64, result.distance(), expected_distance);
            assert_eq!(result.path(), expected_path);
        });
}

#[test]
fn fastdtw_with_distance_closure() {
    include_str!("fastdtw_test_cases.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(6)
        .for_each(|chunk| {
            let a = into_float_vec::<f64>(chunk[0]);
            let b = into_float_vec::<f64>(chunk[1]);
            let expected_path = into_float_vec::<usize>(chunk[2])
                .into_iter()
                .zip(into_float_vec::<usize>(chunk[3]))
                .collect::<Vec<(usize, usize)>>();
            let expected_distance = chunk[4].parse::<f64>().unwrap();
            let radius = chunk[5].parse::<usize>().unwrap();

            let result =
                fastdtw_with_distance(&a, &b, radius, |a, b| f64::abs(a - b), |a, b| (a + b) / 2.0);

            assert_approx_eq!(f64, result.distance(), expected_distance);
            assert_eq!(result.path(), expected_path);
        });
}

#[test]
fn fastdtw_with_distance_trait() {
    include_str!("fastdtw_test_cases.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(6)
        .for_each(|chunk| {
            let a = into_float_vec::<f64>(chunk[0]);
            let b = into_float_vec::<f64>(chunk[1]);
            let expected_path = into_float_vec::<usize>(chunk[2])
                .into_iter()
                .zip(into_float_vec::<usize>(chunk[3]))
                .collect::<Vec<(usize, usize)>>();
            let expected_distance = chunk[4].parse::<f64>().unwrap();
            let radius = chunk[5].parse::<usize>().unwrap();

            let result = fastdtw(&a, &b, radius);

            assert_approx_eq!(f64, result.distance(), expected_distance);
            assert_eq!(result.path(), expected_path);
        });
}

>>>>>>> feature/new-structure
#[inline]
fn into_float_vec<T: FromStr>(line: &str) -> Vec<T> {
    line.split(' ')
        .map(str::parse::<T>)
        .filter_map(Result::ok)
        .collect::<Vec<T>>()
}

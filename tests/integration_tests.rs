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

#[inline]
fn into_float_vec<T: FromStr>(line: &str) -> Vec<T> {
    line.split(' ')
        .map(str::parse::<T>)
        .filter_map(Result::ok)
        .collect::<Vec<T>>()
}

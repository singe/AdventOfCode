use one::{parse, diff};

#[test]
fn test_diff_equal_vectors() {
    let vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![1, 2, 3, 4];
    let result = diff(&vec1, &vec2);
    assert_eq!(result, 0); // No difference
}

#[test]
fn test_diff_different_vectors() {
    let vec1 = vec![3, 1, 4, 2];
    let vec2 = vec![5, 3, 9, 3];
    let result = diff(&vec1, &vec2);
    assert_eq!(result, 10); // Differences: [2, 2, 5, 1] -> 10
}

#[test]
fn test_diff_with_negatives() {
    let vec1 = vec![-1, -2, -3, -4];
    let vec2 = vec![4, 3, 2, 1];
    let result = diff(&vec1, &vec2);
    assert_eq!(result, 20); // Differences: [1+4, 2+3, 3+2, 4+1] -> 20
}

#[test]
fn test_diff_unbalanced_vectors() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let result = diff(&vec1, &vec2);
    assert_eq!(result, 9); // Differences: [3, 3, 3] -> 9
}

#[test]
fn test_diff_different_lengths() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5];

    // Catch the panic
    let result = std::panic::catch_unwind(|| diff(&vec1, &vec2));
    assert!(result.is_err(), "diff did not panic on vectors of different lengths");
}

#[test]
fn test_parse_valid_file() {
    // Create test data and write it to a temporary file
    let test_data = "1 2\n3 4\n5 6\n";
    let path = "test_parse_valid.txt";
    std::fs::write(path, test_data).unwrap();

    let result = parse(path);
    std::fs::remove_file(path).unwrap(); // Clean up test file

    match result {
        Ok((vec1, vec2)) => {
            assert_eq!(vec1, vec![1, 3, 5]);
            assert_eq!(vec2, vec![2, 4, 6]);
        }
        Err(_) => panic!("parse failed on valid input"),
    }
}

#[test]
fn test_parse_invalid_file() {
    // Create test data
    let test_data = "1 2\ninvalid line\n5 6\n";
    let path = "test_parse_invalid.txt";

    // Write to a temporary file
    std::fs::write(path, test_data).unwrap();

    // Test parse
    let result = parse(path);
    std::fs::remove_file(path).unwrap(); // Clean up test file

    match result {
        Ok((vec1, vec2)) => {
            assert_eq!(vec1, vec![1, 5]); // Valid lines only
            assert_eq!(vec2, vec![2, 6]); // Valid lines only
        }
        Err(_) => panic!("parse failed on valid input"),
    }
}

use one::{frequency, frequency_sum};
use std::collections::HashMap;

#[test]
fn test_frequency_basic() {
    let numbers = vec![4, 3, 5, 3, 9, 3];
    let freq_map = frequency(&numbers);

    let mut expected_map = HashMap::new();
    expected_map.insert(3, 3);
    expected_map.insert(4, 1);
    expected_map.insert(5, 1);
    expected_map.insert(9, 1);

    assert_eq!(freq_map, expected_map);
}

#[test]
fn test_frequency_sum_basic() {
    let numbers = vec![4, 3, 5, 3, 9, 3];
    let freq_map = frequency(&numbers);
    let sum = frequency_sum(&numbers, &freq_map);

    // Expected:
    // 4 * 1 + 3 * 3 + 5 * 1 + 3 * 3 + 9 * 1 + 3 * 3 = 45
    assert_eq!(sum, 45);
}
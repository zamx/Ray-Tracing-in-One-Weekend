use crate::data::vec3::{cross, dot, Vec3};
use float_eq::assert_float_eq;

#[test]
fn new_test() {
    let vec = Vec3::new(1.0, 2.5, 4.7);

    assert_float_eq!(vec.x(), 1.0, r2nd <= 1e-10);
    assert_float_eq!(vec.y(), 2.5, r2nd <= 1e-10);
    assert_float_eq!(vec.z(), 4.7, r2nd <= 1e-10);
}

#[test]
fn length_test() {
    let vec = Vec3::new(1.0, 2.5, 4.7);

    assert_float_eq!(vec.length(), 5.41664102558, r2nd <= 1e-10);
}

#[test]
fn squared_length_test() {
    let vec = Vec3::new(1.0, 2.5, 4.7);

    assert_float_eq!(vec.squared_length(), 29.34, r2nd <= 1e-10);
}

#[test]
fn addition_test() {
    let vec1 = Vec3::new(1.0, 2.5, 4.7);
    let vec2 = Vec3::new(0.1, 9.4, 12.8);
    let result = vec1 + vec2;

    assert_float_eq!(result.x(), 1.1, r2nd <= 1e-10);
    assert_float_eq!(result.y(), 11.9, r2nd <= 1e-10);
    assert_float_eq!(result.z(), 17.5, r2nd <= 1e-10);
}

#[test]
fn subtract_test() {
    let vec1 = Vec3::new(1.0, 2.5, 4.7);
    let vec2 = Vec3::new(0.1, 9.4, 12.8);
    let result = vec1 - vec2;

    assert_float_eq!(result.x(), 0.9, r2nd <= 1e-10);
    assert_float_eq!(result.y(), -6.9, r2nd <= 1e-10);
    assert_float_eq!(result.z(), -8.1, r2nd <= 1e-10);
}

#[test]
fn multiplication_double_test() {
    let vec1 = Vec3::new(1.0, 2.5, 4.7);
    let mut result = vec1 * 5.0;

    assert_float_eq!(result.x(), 5.0, r2nd <= 1e-10);
    assert_float_eq!(result.y(), 12.5, r2nd <= 1e-10);
    assert_float_eq!(result.z(), 23.5, r2nd <= 1e-10);

    result = 5.0 * vec1;

    assert_float_eq!(result.x(), 5.0, r2nd <= 1e-10);
    assert_float_eq!(result.y(), 12.5, r2nd <= 1e-10);
    assert_float_eq!(result.z(), 23.5, r2nd <= 1e-10);

    result = vec1 * Vec3::new(5.0, 5.0, 5.0);

    assert_float_eq!(result.x(), 5.0, r2nd <= 1e-10);
    assert_float_eq!(result.y(), 12.5, r2nd <= 1e-10);
    assert_float_eq!(result.z(), 23.5, r2nd <= 1e-10);
}

#[test]
fn divide_double_test() {
    let vec1 = Vec3::new(1.0, 2.5, 4.5);
    let mut result = vec1 / 5.0;

    assert_float_eq!(result.x(), 0.2, r2nd <= 1e-10);
    assert_float_eq!(result.y(), 0.5, r2nd <= 1e-10);
    assert_float_eq!(result.z(), 0.9, r2nd <= 1e-10);

    result = vec1;
    result /= 5.0;

    assert_float_eq!(result.x(), 0.2, r2nd <= 1e-10);
    assert_float_eq!(result.y(), 0.5, r2nd <= 1e-10);
    assert_float_eq!(result.z(), 0.9, r2nd <= 1e-10);
}

#[test]
fn negation_test() {
    let vec1 = Vec3::new(1.0, 2.5, 4.5);
    let result = -vec1;

    assert_float_eq!(result.x(), -1.0, r2nd <= 1e-10);
    assert_float_eq!(result.y(), -2.5, r2nd <= 1e-10);
    assert_float_eq!(result.z(), -4.5, r2nd <= 1e-10);
}

#[test]
fn dot_test() {
    let vec1 = Vec3::new(1.0, 2.5, 4.7);
    let vec2 = Vec3::new(0.1, 9.4, 12.8);
    let result = dot(&vec1, &vec2);

    assert_float_eq!(result, 83.76, r2nd <= 1e-10);
}

#[test]
fn cross_test() {
    let vec1 = Vec3::new(1.0, 2.5, 4.7);
    let vec2 = Vec3::new(0.1, 9.4, 12.8);
    let result = cross(&vec1, &vec2);

    assert_float_eq!(result.x(), -12.18, r2nd <= 1e-10);
    assert_float_eq!(result.y(), -12.33, r2nd <= 1e-10);
    assert_float_eq!(result.z(), 9.15, r2nd <= 1e-10);
}

#[test]
fn index_test() {
    let vec1 = Vec3::new(1.0, 2.5, 4.7);

    assert_float_eq!(vec1[0], 1.0, r2nd <= 1e-10);
    assert_float_eq!(vec1[1], 2.5, r2nd <= 1e-10);
    assert_float_eq!(vec1[2], 4.7, r2nd <= 1e-10);
}

#[test]
#[should_panic(expected = "index out of range 3")]
fn index_should_panic_test() {
    let vec1 = Vec3::new(1.0, 2.5, 4.7);

    vec1[3];
}
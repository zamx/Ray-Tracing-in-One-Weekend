use crate::data::color::Color;

#[test]
fn test_new() {
    let color = Color::new(0, 1, 2);
    assert_eq!(color.r(), 0);
    assert_eq!(color.g(), 1);
    assert_eq!(color.b(), 2);
}

#[test]
fn multiplication_int_test() {
    let color = Color::new(1, 2, 4);
    let mut result = 5 * color;

    assert_eq!(result.r(), 5);
    assert_eq!(result.g(), 10);
    assert_eq!(result.b(), 20);

    result = color * 5;

    assert_eq!(result.r(), 5);
    assert_eq!(result.g(), 10);
    assert_eq!(result.b(), 20);
}

#[test]
fn multiplication_double_test() {
    let color = Color::new(1, 2, 4);
    let mut result = color * 5.0;

    assert_eq!(result.r(), 5);
    assert_eq!(result.g(), 10);
    assert_eq!(result.b(), 20);

    result = 5.0 * color;

    assert_eq!(result.r(), 5);
    assert_eq!(result.g(), 10);
    assert_eq!(result.b(), 20);
}
use crate::data::ray::Ray;
use crate::data::vec3::Vec3;

fn create_ray() -> Ray {
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let direction = Vec3::new(0.0, 0.0, -1.0);

    Ray::new(origin, direction)
}

#[test]
fn new_test() {
    let ray = create_ray();

    assert_eq!(ray.origin(), &Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(ray.direction(), & Vec3::new(0.0, 0.0, -1.0));
}

#[test]
fn point_at_test() {
    let ray = create_ray();

    assert_eq!(ray.point_at(1.5), Vec3::new(0.0, 0.0, -1.5));
}
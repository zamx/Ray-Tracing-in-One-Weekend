use crate::data::vec3::Vec3;

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3
}

impl HitRecord {
    pub fn new(t: f64, point: Vec3, normal: Vec3) -> HitRecord {
        HitRecord{ t, point, normal }
    }
}
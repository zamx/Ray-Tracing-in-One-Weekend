use crate::data::color::Color;
use crate::data::ray::Ray;

pub trait RayTraceObject {
    fn hit(&self, ray: &Ray) -> Option<Color>;
}
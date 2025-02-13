use crate::data::color::Color;
use crate::data::ray::Ray;
use crate::visuals::hit_record::HitRecord;

pub trait RayTraceObject {
    fn hit(&self, ray: &Ray, ray_t_min: f64, ray_t_max: f64) -> Option<HitRecord>;

    fn color_at(&self, hit: &HitRecord) -> Option<Color>;
}
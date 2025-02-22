use crate::data::color::Color;
use crate::data::interval::Interval;
use crate::data::ray::Ray;
use crate::visuals::hit_record::HitRecord;

pub trait RayTraceObject {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn color_at(&self, hit: &HitRecord) -> Option<Color>;
}
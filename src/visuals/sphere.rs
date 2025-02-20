use std::cmp::max;
use crate::data::{color, vec3};
use crate::data::vec3::Vec3;
use crate::data::color::Color;
use crate::data::ray::Ray;
use crate::visuals::hit_record::HitRecord;
use crate::visuals::ray_trace_object::RayTraceObject;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    color: Color,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, color: Color) -> Sphere {
        Sphere {center, radius, color}
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

fn unit_to_range(value: f64, min: f64, max: f64) -> f64 {
    assert!(min <= max);

    let range = max - min;

    ((value * range) + range) / 2.0
}

impl RayTraceObject for Sphere {
    fn hit(&self, ray: &Ray, ray_t_min: f64, ray_t_max: f64) -> Option<HitRecord> {
        let oc = self.center - *ray.origin();
        let a = ray.direction().squared_length();
        let h = vec3::dot(ray.direction(), &oc);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (h - sqrt_d) / a;
        if root < ray_t_min || ray_t_max < root {
            root = (h + sqrt_d) / a;
            if root <= ray_t_min || ray_t_max > root {
                return None;
            }
        }

        Some(HitRecord::new(root, ray, self))
    }

    fn color_at(&self, hit: &HitRecord) -> Option<Color> {
        match hit.t() > 0.0 {
            true => {
                let color = Color::new(
                    unit_to_range(hit.normal().x(), 0.0, 255.0) as i32,
                    unit_to_range(hit.normal().y(), 0.0, 255.0) as i32,
                    unit_to_range(hit.normal().z(), 0.0, 255.0) as i32
                );
                Some(color)
            },
            false => None
        }
    }
}
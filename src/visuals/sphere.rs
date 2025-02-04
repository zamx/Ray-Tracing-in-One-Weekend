use crate::data::vec3;
use crate::data::vec3::Vec3;
use crate::data::color::Color;
use crate::data::ray::Ray;
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

impl RayTraceObject for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Color> {
        let oc = self.center - *ray.origin();
        let a = vec3::dot(ray.direction(), ray.direction());
        let b = 2.0 * vec3::dot(&oc, ray.direction());
        let c = vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        match discriminant >= 0.0 {
            true => Some(self.color),
            false => None
        }
    }
}
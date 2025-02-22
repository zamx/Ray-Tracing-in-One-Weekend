use crate::data::ray::Ray;
use crate::data::vec3;
use crate::data::vec3::Vec3;
use crate::visuals::sphere::Sphere;

pub struct HitRecord {
    t: f64,
    point: Vec3,
    normal: Vec3,
    front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, ray: &Ray, sphere: &Sphere) -> HitRecord {
        let point = ray.point_at(t);
        let outward_normal = (point - sphere.center()) / sphere.radius();
        let (front_face, normal) = Self::get_face_normal(ray, outward_normal);

        HitRecord{ t, point, normal, front_face }
    }

    pub const fn t(&self) -> f64 {
        self.t
    }

    pub const fn normal(&self) -> &Vec3 {
        &self.normal
    }

    fn get_face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = vec3::dot(ray.direction(), &outward_normal) < 0.0;

        if let front_face = true {
            return (front_face, outward_normal);
        }

        (front_face, -outward_normal)
    }
}
use crate::data::color::Color;
use crate::data::ray::Ray;
use crate::visuals::ray_trace_object::RayTraceObject;

pub struct Scene {
    objects: Vec<Box<dyn RayTraceObject>>,
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn RayTraceObject>>) -> Scene {
        Scene { objects }
    }

    pub fn empty() -> Scene {
        Scene { objects :  Vec::new() }
    }

    pub fn add_object(&mut self, object: impl RayTraceObject + 'static) {
        self.objects.push(Box::new(object));
    }

    pub fn cast(&self, ray: &Ray) -> Option<Color> {
        for object in &self.objects {
            let hit = object.hit(ray, 0.0, f64::INFINITY);

            match hit {
                Some(hit) => return object.color_at(&hit),
                _ => ()
            }
        }

        return None;
    }
}
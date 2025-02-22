use crate::data::color::Color;
use crate::data::vec3::Vec3;
use crate::visuals::{sphere, scene};
use crate::visuals::scene::Scene;

pub fn create_scene() -> scene::Scene {
    let sphere1 = sphere::Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5, Color::red());
    let sphere2 = sphere::Sphere::new(Vec3::new(0.75,0.75,-2.0), 0.2, Color::blue());
    let sphere3 = sphere::Sphere::new(Vec3::new(-0.75,-0.75,-2.0), 0.2, Color::green());
    let sphere4 = sphere::Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0, Color::green());

    Scene::new(vec![Box::new(sphere1), Box::new(sphere2), Box::new(sphere3), Box::new(sphere4)])
}
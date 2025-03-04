use crate::data::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub const fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub const fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
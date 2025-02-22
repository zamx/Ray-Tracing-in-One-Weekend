use std::ops;
use std::ops::Div;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    data: [f64; 3]
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {data: [x, y, z]}
    }

    pub const fn zero() -> Vec3 {
        Vec3 {data: [0.0, 0.0, 0.0]}
    }

    pub const fn x(&self) -> f64 {
        self.data[0]
    }

    pub const fn y(&self) -> f64 {
        self.data[1]
    }

    pub const fn z(&self) -> f64 {
        self.data[2]
    }

    pub fn length(&self) -> f64 {
       self.squared_length().sqrt()
    }

    pub const fn squared_length(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
}

pub const fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
}

pub const fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    Vec3::new(
        lhs.y() * rhs.z() - lhs.z() * rhs.y(),
        lhs.z() * rhs.x() - lhs.x() * rhs.z(),
        lhs.x() * rhs.y() - lhs.y() * rhs.x()
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        match idx {
            0..=2 => &self.data[idx],
            _ => panic!("index out of range {}", idx),
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<u32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: u32) -> Vec3 {
        let double = other as f64;
        Vec3::new(self.x() * double, self.y() * double, self.z() * double)
    }
}

impl ops::Mul<Vec3> for u32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x() * other.x(), self.y() * other.y(), self.z() * other.y())
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3::new(self.x() / other, self.y() / other, self.z() / other)
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        *self / rhs
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.data[0] /= rhs;
        self.data[1] /= rhs;
        self.data[2] /= rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}
use std::fmt::Debug;
use std::ops;
use std::ops::Add;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Color {
    data: [i32; 3]
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Color {
        let data = [r, g, b];
        Color { data }
    }

    pub fn black() -> Color {
        Color::new( 0, 0, 0 )
    }

    pub fn white() -> Color { Color::new(255, 255, 255) }

    pub fn red() -> Color {
        Color::new( 255, 0, 0 )
    }

    pub fn green() -> Color {
        Color::new( 0, 255, 0 )
    }

    pub fn blue() -> Color {
        Color::new(0, 0, 255 )
    }

    pub fn r(&self) -> i32 {
        self.data[0]
    }

    pub fn g(&self) -> i32 {
        self.data[1]
    }

    pub fn b(&self) -> i32 {
        self.data[2]
    }
}

impl ops::Mul<i32> for Color {
    type Output = Color;

    fn mul(self, other: i32) -> Color {
        Color::new(self.r() * other, self.g() * other, self.b() * other)
    }
}

impl ops::Mul<Color> for i32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color  {
        Color::new( (self.data[0] as f64 * other) as i32, (self.data[1] as f64 * other) as i32, (self.data[2] as f64 * other) as i32  )
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.data[0] = (self.data[0] as f64 * rhs) as i32;
        self.data[1] = (self.data[1] as f64 * rhs) as i32;
        self.data[2] = (self.data[2] as f64 * rhs) as i32;
    }
}

impl ops::MulAssign<i32> for Color {
    fn mul_assign(&mut self, rhs: i32) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.data[0] + rhs.data[0], self.data[1] + rhs.data[1], self.data[2] + rhs.data[2] )
    }
}

use core::ops::{Add, Mul, Sub};
use libm::{cosf, sinf, sqrtf};

/// 2-dimensional vector (generic)
#[derive(Default, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

/// element-wise multiplication
impl<T: Mul<Output = T> + Copy> Mul<T> for &Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// element-wise addition
impl<T: Add<Output = T> + Copy> Add<&Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, rhs: &Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// element-wise substraction
impl<T: Sub<Output = T> + Copy> Sub<&Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: &Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// add lerp & int for f32
impl Vec2<f32> {
    pub fn lerp_to(&mut self, other: &Vec2<f32>, t: f32) {
        self.x += t * (other.x - self.x);
        self.y += t * (other.y - self.y);
    }

    pub fn to_int(&self) -> Vec2<i32> {
        Vec2::<i32> {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    pub fn scale(&mut self, f: f32) {
        self.x *= f;
        self.y *= f;
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy + Into<f32> + From<f32>>
    Vec2<T>
{
    /// get the norm (or the length) of the vector
    pub fn norm(self) -> f32 {
        let x: f32 = self.x.into();
        let y: f32 = self.y.into();
        sqrtf(x * x + y * y)
    }

    /// get the norm (or the length) squared of the vector
    pub fn norm_sqd(self) -> f32 {
        let x: f32 = self.x.into();
        let y: f32 = self.y.into();
        x * x + y * y
    }

    /// normalize the vector
    pub fn normalize(&mut self) {
        let x: f32 = self.x.into();
        let y: f32 = self.y.into();
        let inv_sqrt_norm = 1. / sqrtf(x * x + y * y);
        self.x = T::from(x * inv_sqrt_norm);
        self.y = T::from(y * inv_sqrt_norm);
    }

    /// return a normalized copy of the vector
    pub fn normalized(self) -> Self {
        let x: f32 = self.x.into();
        let y: f32 = self.y.into();
        let inv_sqrt_norm = 1. / sqrtf(x * x + y * y);
        Vec2 {
            x: T::from(x * inv_sqrt_norm),
            y: T::from(y * inv_sqrt_norm),
        }
    }

    /// rotate the vector
    pub fn rotate(&mut self, angle: f32) {
        let x: f32 = self.x.into();
        let y: f32 = self.y.into();
        let cosx = cosf(angle);
        let sinx = sinf(angle);
        self.x = T::from(x * cosx - y * sinx);
        self.y = T::from(x * sinx + y * cosx);
    }

    /// return a rotated copy of the vector
    pub fn rotated(self, angle: f32) -> Self {
        let x: f32 = self.x.into();
        let y: f32 = self.y.into();
        let cosx = cosf(angle);
        let sinx = sinf(angle);
        Vec2 {
            x: T::from(x * cosx - y * sinx),
            y: T::from(x * sinx + y * cosx),
        }
    }

    /// return the dot product of the vectors
    pub fn dot(self, other: Self) -> f32 {
        let x1: f32 = self.x.into();
        let y1: f32 = self.y.into();
        let x2: f32 = other.x.into();
        let y2: f32 = other.y.into();
        x1 * x2 + y1 * y2
    }
}

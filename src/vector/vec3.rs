use crate::float_near;
use crate::vector::{Vec2, Vec4};
use std::cmp;
use std::ops;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn mag(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit(self) -> Vec3 {
        self / self.mag()
    }
}

impl<T: Into<Vec3>> ops::Add<T> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Into<Vec3>> ops::Sub<T> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Self::Output {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl ops::Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: i32) -> Self::Output {
        Vec3::new(
            self.x * scalar as f32,
            self.y * scalar as f32,
            self.z * scalar as f32,
        )
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Self::Output {
        Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl ops::Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: i32) -> Self::Output {
        Vec3::new(
            self.x / scalar as f32,
            self.y / scalar as f32,
            self.z / scalar as f32,
        )
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        float_near(self.x, other.x) && float_near(self.y, other.y) && float_near(self.z, other.z)
    }
}

impl From<Vec2> for Vec3 {
    fn from(vec2: Vec2) -> Vec3 {
        Vec3 {
            x: vec2.x,
            y: vec2.y,
            z: 0.0,
        }
    }
}

impl From<Vec4> for Vec3 {
    fn from(vec: Vec4) -> Vec3 {
        Vec3::new(vec.x, vec.y, vec.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        let lhs = Vec3::new(1.0, 2.0, 3.0);
        let rhs = Vec3::new(3.0, 5.0, 7.0);
        let added = lhs + rhs;
        let subbed = lhs - rhs;
        let vec2_added = lhs + Vec2::from(rhs);
        let vec2_subbed = lhs - Vec2::from(rhs);
        let cross = lhs.cross(rhs);
        let dot = lhs.dot(rhs);
        let mag = rhs.mag();
        // let between = lhs.between(&rhs);
        let int_scale_mult = lhs * 2;
        let int_scale_div = lhs / 2;
        let float_scale_mult = rhs * 2.5;
        let float_scale_div = rhs / 2.5;
        let unit = lhs.unit();

        assert_eq!(added, Vec3::new(4.0, 7.0, 10.0));
        assert_eq!(subbed, Vec3::new(-2.0, -3.0, -4.0));
        assert_eq!(vec2_added, Vec3::new(4.0, 7.0, 3.0));
        assert_eq!(vec2_subbed, Vec3::new(-2.0, -3.0, 3.0));
        assert_eq!(cross, Vec3::new(-1.0, 2.0, -1.0));
        assert!(float_near(dot, 34.0));
        assert!(float_near(mag, 9.110433));
        assert_eq!(int_scale_mult, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(int_scale_div, Vec3::new(0.5, 1.0, 1.5));;
        assert_eq!(float_scale_mult, Vec3::new(7.5, 12.5, 17.5));
        assert_eq!(float_scale_div, Vec3::new(1.2, 2.0, 2.8));
        assert_eq!(unit, Vec3::new(0.267261, 0.534522, 0.801783));
    }
}

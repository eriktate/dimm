use crate::float_near;
use crate::vector::{Vec2, Vec3};
use std::cmp;
use std::ops;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { x, y, z, w }
    }

    pub fn dot(self, rhs: Vec4) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn mag(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn unit(self) -> Vec4 {
        self / self.mag()
    }
}

impl<T: Into<Vec4>> ops::Add<T> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl<T: Into<Vec4>> ops::Sub<T> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vec4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl ops::Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, scalar: f32) -> Self::Output {
        Vec4::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl ops::Mul<i32> for Vec4 {
    type Output = Vec4;

    fn mul(self, scalar: i32) -> Self::Output {
        Vec4::new(
            self.x * scalar as f32,
            self.y * scalar as f32,
            self.z * scalar as f32,
            self.w * scalar as f32,
        )
    }
}

impl ops::Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, scalar: f32) -> Self::Output {
        Vec4::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
    }
}

impl ops::Div<i32> for Vec4 {
    type Output = Vec4;

    fn div(self, scalar: i32) -> Self::Output {
        Vec4::new(
            self.x / scalar as f32,
            self.y / scalar as f32,
            self.z / scalar as f32,
            self.w / scalar as f32,
        )
    }
}

impl cmp::PartialEq for Vec4 {
    fn eq(&self, other: &Vec4) -> bool {
        float_near(self.x, other.x)
            && float_near(self.y, other.y)
            && float_near(self.z, other.z)
            && float_near(self.w, other.w)
    }
}

impl From<Vec2> for Vec4 {
    fn from(vec: Vec2) -> Vec4 {
        Vec4::new(vec.x, vec.y, 0.0, 1.0)
    }
}

impl From<Vec3> for Vec4 {
    fn from(vec: Vec3) -> Vec4 {
        Vec4::new(vec.x, vec.y, vec.z, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        let lhs = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let rhs = Vec4::new(3.0, 5.0, 7.0, 9.0);
        let added = lhs + rhs;
        // let subbed = lhs - rhs;
        // let vec2_added = lhs + Vec2::from(rhs);
        // let vec2_subbed = lhs - Vec2::from(rhs);
        // let cross = lhs.cross(rhs);
        // let dot = lhs.dot(rhs);
        // let mag = rhs.mag();
        // // let between = lhs.between(&rhs);
        // let int_scale_mult = lhs * 2;
        // let int_scale_div = lhs / 2;
        // let float_scale_mult = rhs * 2.5;
        // let float_scale_div = rhs / 2.5;
        // let unit = lhs.unit();

        assert_eq!(added, Vec4::new(4.0, 7.0, 10.0, 13.0));
        // assert_eq!(subbed, Vec3::new(-2.0, -3.0, -4.0));
        // assert_eq!(cross, Vec3::new(-1.0, 2.0, -1.0));
        // assert!(float_near(dot, 34.0));
        // assert!(float_near(mag, 9.110433));
        // assert_eq!(int_scale_mult, Vec3::new(2.0, 4.0, 6.0));
        // assert_eq!(int_scale_div, Vec3::new(0.5, 1.0, 1.5));;
        // assert_eq!(float_scale_mult, Vec3::new(7.5, 12.5, 17.5));
        // assert_eq!(float_scale_div, Vec3::new(1.2, 2.0, 2.8));
        // assert_eq!(unit, Vec3::new(0.267261, 0.534522, 0.801783));
    }
}

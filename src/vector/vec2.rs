use crate::float_near;
use crate::vector::{Vec3, Vec4};
use std::cmp;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn dot(self, rhs: Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(self, rhs: Vec2) -> Vec3 {
        let lhs = Vec3::from(self);
        let rhs = Vec3::from(rhs);

        lhs.cross(rhs)
    }

    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn angle(&self) -> f32 {
        (self.y / self.x).atan()
    }

    pub fn between(self, rhs: Vec2) -> f32 {
        (self.dot(rhs) / (self.mag() * rhs.mag())).acos()
    }

    pub fn unit(self) -> Vec2 {
        self / self.mag()
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl ops::Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: i32) -> Vec2 {
        Vec2 {
            x: self.x * scalar as f32,
            y: self.y * scalar as f32,
        }
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl ops::Div<i32> for Vec2 {
    type Output = Vec2;

    fn div(self, scalar: i32) -> Vec2 {
        Vec2 {
            x: self.x / scalar as f32,
            y: self.y / scalar as f32,
        }
    }
}

impl cmp::PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        float_near(self.x, other.x) && float_near(self.y, other.y)
    }
}

impl From<Vec3> for Vec2 {
    fn from(vec: Vec3) -> Vec2 {
        Vec2::new(vec.x, vec.y)
    }
}

impl From<Vec4> for Vec2 {
    fn from(vec: Vec4) -> Vec2 {
        Vec2::new(vec.x, vec.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        let lhs = Vec2::new(1.0, 2.0);
        let rhs = Vec2::new(3.0, 5.0);
        let added = lhs + rhs;
        let subbed = lhs - rhs;
        let cross = lhs.cross(rhs);
        let dot = lhs.dot(rhs);
        let mag = rhs.mag();
        let angle = lhs.angle();
        // let between = lhs.between(&rhs);
        let int_scale_mult = lhs * 2;
        let int_scale_div = lhs / 2;
        let float_scale_mult = rhs * 2.5;
        let float_scale_div = rhs / 2.5;
        let unit = lhs.unit();

        assert_eq!(added, Vec2::new(4.0, 7.0));
        assert_eq!(subbed, Vec2::new(-2.0, -3.0));
        assert_eq!(cross, Vec3::new(0.0, 0.0, -1.0));
        assert!(float_near(dot, 13.0));
        assert!(float_near(mag, 5.830951));
        assert!(float_near(angle, 1.107148));
        // assert!(float_near(between, 0.997054));
        assert_eq!(int_scale_mult, Vec2::new(2.0, 4.0));
        assert_eq!(int_scale_div, Vec2::new(0.5, 1.0));;
        assert_eq!(float_scale_mult, Vec2::new(7.5, 12.5));
        assert_eq!(float_scale_div, Vec2::new(1.2, 2.0));
        assert_eq!(unit, Vec2::new(0.447213, 0.894427));
    }
}

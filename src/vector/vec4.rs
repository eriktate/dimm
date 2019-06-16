use crate::vector::{Vec2, Vec3};

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

    pub fn dot(&self, rhs: &Vec4) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
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

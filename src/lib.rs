use std::cmp;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Mat4([f32; 16]);

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn dot(&self, rhs: &Vec2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(&self, rhs: &Vec2) -> Vec3 {
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

    pub fn between(&self, rhs: &Vec2) -> f32 {
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

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        float_near(self.x, other.x) && float_near(self.y, other.y) && float_near(self.z, other.z)
    }
}

impl From<&Vec2> for Vec3 {
    fn from(vec2: &Vec2) -> Vec3 {
        Vec3 {
            x: vec2.x,
            y: vec2.y,
            z: 0.0,
        }
    }
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

impl Mat4 {
    pub fn new() -> Mat4 {
        Mat4([0.0; 16])
    }

    pub fn identity() -> Mat4 {
        Mat4([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn translation(translation: Vec3) -> Mat4 {
        let mut mat = Mat4::identity().0;
        mat[3] += translation.x;
        mat[7] += translation.y;
        mat[11] += translation.z;

        Mat4(mat)
    }

    pub fn scaling(scaling: Vec3) -> Mat4 {
        let mut mat = Mat4::identity().0;
        mat[0] = scaling.x;
        mat[5] = scaling.y;
        mat[9] = scaling.z;

        Mat4(mat)
    }

    pub fn row(&self, row: usize) -> Vec4 {
        Vec4::new(
            self[row * 4],
            self[row * 4 + 1],
            self[row * 4 + 2],
            self[row * 4 + 3],
        )
    }

    pub fn col(&self, column: usize) -> Vec4 {
        Vec4::new(
            self[column],
            self[4 + column],
            self[8 + column],
            self[12 + column],
        )
    }
}

impl ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        let rows: [Vec4; 4] = [self.row(0), self.row(1), self.row(2), self.row(3)];
        let cols: [Vec4; 4] = [rhs.col(0), rhs.col(1), rhs.col(2), rhs.col(3)];
        let mut mat = Mat4::new();

        for i in 0..4 {
            mat[i * 4] = rows[i].dot(&cols[0]);
            mat[i * 4 + 1] = rows[i].dot(&cols[1]);
            mat[i * 4 + 2] = rows[i].dot(&cols[2]);
            mat[i * 4 + 3] = rows[i].dot(&cols[3]);
        }

        mat
    }
}

impl<T: From<Vec4> + Into<Vec4>> ops::Mul<T> for Mat4 {
    type Output = T;
    fn mul(self, rhs: T) -> Self::Output {
        let vec = rhs.into();

        T::from(Vec4::new(
            self.row(0).dot(&vec),
            self.row(1).dot(&vec),
            self.row(2).dot(&vec),
            self.row(3).dot(&vec),
        ))
    }
}

impl ops::Index<usize> for Mat4 {
    type Output = f32;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl ops::IndexMut<usize> for Mat4 {
    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Self::Output {
        &mut self.0[idx]
    }
}

impl std::fmt::Display for Mat4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}\n{:?}\n{:?}\n{:?}\n",
            self.row(0),
            self.row(1),
            self.row(2),
            self.row(3)
        )
    }
}

// an approximation of float equality. f32 should be precise up to at least 6 significant digits
fn float_near(first: f32, second: f32) -> bool {
    (first - second).abs() < 0.000001
}

pub fn degtorad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

pub fn radtodeg(rad: f32) -> f32 {
    rad * 180.0 / std::f32::consts::PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2() {
        let lhs = Vec2::new(1.0, 2.0);
        let rhs = Vec2::new(3.0, 5.0);
        let added = lhs + rhs;
        let subbed = lhs - rhs;
        let cross = lhs.cross(&rhs);
        let dot = lhs.dot(&rhs);
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

    #[test]
    fn test_transformation() {
        let translation = Vec3::new(1.0, 2.0, 0.0);
        let scaling = Vec3::new(2.0, 2.0, 0.0);

        let mut trans = Mat4::translation(translation) * Mat4::scaling(scaling);
        let vec = Vec2::new(2.0, 5.0);

        let translated = trans * vec;
        assert_eq!(translated, Vec2::new(5.0, 12.0));
    }
}

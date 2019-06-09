use std::cmp;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn from_ints(x: i32, y: i32) -> Vector2 {
        Vector2 {
            x: x as f32,
            y: y as f32,
        }
    }

    pub fn dot(&self, rhs: &Vector2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(&self, rhs: &Vector2) -> Vector3 {
        let lhs = Vector3::from(self);
        let rhs = Vector3::from(rhs);

        lhs.cross(rhs)
    }

    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn angle(&self) -> f32 {
        (self.y / self.x).atan()
    }

    pub fn between(&self, rhs: &Vector2) -> f32 {
        (self.dot(rhs) / (self.mag() * rhs.mag())).acos()
    }

    pub fn unit(self) -> Vector2 {
        self / self.mag()
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl ops::Mul<i32> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: i32) -> Vector2 {
        Vector2 {
            x: self.x * scalar as f32,
            y: self.y * scalar as f32,
        }
    }
}

impl ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl ops::Div<i32> for Vector2 {
    type Output = Vector2;

    fn div(self, scalar: i32) -> Vector2 {
        Vector2 {
            x: self.x / scalar as f32,
            y: self.y / scalar as f32,
        }
    }
}

impl cmp::PartialEq for Vector2 {
    fn eq(&self, other: &Vector2) -> bool {
        float_near(self.x, other.x) && float_near(self.y, other.y)
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn from_ints(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }

    pub fn cross(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl cmp::PartialEq for Vector3 {
    fn eq(&self, other: &Vector3) -> bool {
        float_near(self.x, other.x) && float_near(self.y, other.y) && float_near(self.z, other.z)
    }
}

impl From<&Vector2> for Vector3 {
    fn from(vec2: &Vector2) -> Vector3 {
        Vector3 {
            x: vec2.x,
            y: vec2.y,
            z: 0.0,
        }
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
        let lhs = Vector2::from_ints(1, 2);
        let rhs = Vector2::from_ints(3, 5);
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

        assert_eq!(added, Vector2::from_ints(4, 7));
        assert_eq!(subbed, Vector2::from_ints(-2, -3));
        assert_eq!(cross, Vector3::from_ints(0, 0, -1));
        assert!(float_near(dot, 13.0));
        assert!(float_near(mag, 5.830951));
        assert!(float_near(angle, 1.107148));
        // assert!(float_near(between, 0.997054));
        assert_eq!(int_scale_mult, Vector2::from_ints(2, 4));
        assert_eq!(int_scale_div, Vector2::new(0.5, 1.0));;
        assert_eq!(float_scale_mult, Vector2::new(7.5, 12.5));
        assert_eq!(float_scale_div, Vector2::new(1.2, 2.0));
        assert_eq!(unit, Vector2::new(0.447213, 0.894427));
    }
}

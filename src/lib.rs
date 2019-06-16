pub mod matrix;
pub mod vector;

// an approximation of float equality. f32 should be precise up to at least 6 significant digits
pub fn float_near(first: f32, second: f32) -> bool {
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
    use super::matrix::Mat4;
    use super::vector::{Vec2, Vec3, Vec4};
    use super::*;

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

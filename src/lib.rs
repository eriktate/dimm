pub mod matrix;
pub mod vector;

/// An approximation of float equality. f32 should be precise up to at least 6 significant digits
/// This is mostly a helper for implementing the `PartialEq` trait on vector types.
pub fn float_near(first: f32, second: f32) -> bool {
    (first - second).abs() < 0.000001
}

/// Convert from degrees to radians.
pub fn degtorad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

/// Convert from radians to degrees.
pub fn radtodeg(rad: f32) -> f32 {
    rad * 180.0 / std::f32::consts::PI
}

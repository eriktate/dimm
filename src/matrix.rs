use crate::vector::{Vec3, Vec4};
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Mat4([f32; 16]);

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

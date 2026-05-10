use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use rebound_bind as rb;

use crate::types::Rotation;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Vec3d(pub f64, pub f64, pub f64);

impl Vec3d {
    pub fn rotate(self, rotation: Rotation) -> Self {
        let mut r = self;
        r.irotate(rotation);
        r
    }

    pub fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn normalize(self) -> Self {
        self * (1.0 / self.length_squared().sqrt())
    }

    pub fn irotate(&mut self, rotation: Rotation) {
        let imag = rotation.imag();
        let t = Vec3d::cross(imag, *self) * 2.;
        let res = *self + t * rotation.r + Vec3d::cross(imag, t);
        *self = res;
    }
}

impl Add for Vec3d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3d {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<f64> for Vec3d {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<Vec3d> for f64 {
    type Output = Vec3d;

    fn mul(self, other: Vec3d) -> Vec3d {
        other * self
    }
}

impl Div<f64> for Vec3d {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl From<rb::reb_vec3d> for Vec3d {
    fn from(value: rb::reb_vec3d) -> Self {
        Self(value.x, value.y, value.z)
    }
}

impl From<Vec3d> for rb::reb_vec3d {
    fn from(value: Vec3d) -> Self {
        rb::reb_vec3d {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl Debug for Vec3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3d (x = {}, y = {}, z = {})", self.0, self.1, self.2)
    }
}

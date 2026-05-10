use std::ops::Mul;

use rebound_bind as rb;

use crate::types::Vec3d;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rotation {
    pub ix: f64,
    pub iy: f64,
    pub iz: f64,
    pub r: f64,
}

impl Rotation {
    pub fn inverse(self) -> Self {
        let scale = 1.0 / self.length_squared();
        let conjugate = self.conjugate();
        Self {
            ix: conjugate.ix * scale,
            iy: conjugate.iy * scale,
            iz: conjugate.iz * scale,
            r: conjugate.r * scale,
        }
    }

    pub fn identity() -> Self {
        Self {
            ix: 0.0,
            iy: 0.0,
            iz: 0.0,
            r: 1.0,
        }
    }

    pub fn normalize(self) -> Self {
        let scale = 1.0 / self.length_squared().sqrt();
        Self {
            ix: self.ix * scale,
            iy: self.iy * scale,
            iz: self.iz * scale,
            r: self.r * scale,
        }
    }

    pub fn conjugate(self) -> Self {
        Self {
            ix: -self.ix,
            iy: -self.iy,
            iz: -self.iz,
            r: self.r,
        }
    }

    pub fn init_from_to(from: Vec3d, to: Vec3d) -> Self {
        unsafe { rb::reb_rotation_init_from_to(from.into(), to.into()) }.into()
    }

    pub fn init_angle_axis(angle: f64, axis: Vec3d) -> Self {
        unsafe { rb::reb_rotation_init_angle_axis(angle, axis.into()) }.into()
    }

    pub fn init_orbit(
        ascending_node_longitude: f64,
        inclination: f64,
        argument_of_periapsis: f64,
    ) -> Self {
        unsafe {
            rb::reb_rotation_init_orbit(
                ascending_node_longitude,
                inclination,
                argument_of_periapsis,
            )
        }
        .into()
    }

    pub fn init_to_new_axes(newz: Vec3d, newx: Vec3d) -> Self {
        unsafe { rb::reb_rotation_init_to_new_axes(newz.into(), newx.into()) }.into()
    }

    pub fn slerp(self, other: Self, t: f64) -> Self {
        unsafe { rb::reb_rotation_slerp(self.into(), other.into(), t) }.into()
    }

    pub fn imag(self) -> Vec3d {
        Vec3d(self.ix, self.iy, self.iz)
    }

    fn dot(self, other: Self) -> f64 {
        self.r * other.r + self.ix * other.ix + self.iy * other.iy + self.iz * other.iz
    }

    fn length_squared(self) -> f64 {
        self.dot(self)
    }
}

impl Mul<Rotation> for Rotation {
    type Output = Self;

    fn mul(self, rhs: Rotation) -> Self::Output {
        Rotation {
            r: self.r * rhs.r - self.ix * rhs.ix - self.iy * rhs.iy - self.iz * rhs.iz,
            ix: self.r * rhs.ix + self.ix * rhs.r + self.iy * rhs.iz - self.iz * rhs.iy,
            iy: self.r * rhs.iy - self.ix * rhs.iz + self.iy * rhs.r + self.iz * rhs.ix,
            iz: self.r * rhs.iz + self.ix * rhs.iy - self.iy * rhs.ix + self.iz * rhs.r,
        }
    }
}

impl From<rb::reb_rotation> for Rotation {
    fn from(value: rb::reb_rotation) -> Self {
        Self {
            ix: value.ix,
            iy: value.iy,
            iz: value.iz,
            r: value.r,
        }
    }
}

impl From<Rotation> for rb::reb_rotation {
    fn from(value: Rotation) -> Self {
        Self {
            ix: value.ix,
            iy: value.iy,
            iz: value.iz,
            r: value.r,
        }
    }
}

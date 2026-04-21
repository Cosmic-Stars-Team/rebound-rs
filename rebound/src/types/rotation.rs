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

use crate::{bind, types::Vec3d};

pub struct Spherical {
    pub mag: f64,
    pub theta: f64,
    pub phi: f64,
}

/// Transformations to vec3d
///
/// # Arguments
///
/// * `mag` - The magnitude of the spherical coordinate.
/// * `theta` - The polar angle of the spherical coordinate.
/// * `phi` - The azimuthal angle of the spherical coordinate.
///
/// # Returns
///
/// A [`Vec3d`] representing the Cartesian coordinates.
pub fn spherical_to_xyz(mag: f64, theta: f64, phi: f64) -> Vec3d {
    unsafe { bind::reb_tools_spherical_to_xyz(mag, theta, phi) }.into()
}

/// Transformations to spherical coordinates
///
/// # Arguments
///
/// * `pos` - The Cartesian coordinates to convert.
///
/// # Returns
///
/// A [`Spherical`] struct representing the spherical coordinates.
pub fn xyz_to_spherical(pos: Vec3d) -> Spherical {
    let mut mag = 0.0;
    let mut theta = 0.0;
    let mut phi = 0.0;
    unsafe { bind::reb_tools_xyz_to_spherical(pos.into(), &mut mag, &mut theta, &mut phi) };
    Spherical { mag, theta, phi }
}

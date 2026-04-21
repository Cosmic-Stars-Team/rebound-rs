use rebound_bind as rb;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vec3d(pub f64, pub f64, pub f64);

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

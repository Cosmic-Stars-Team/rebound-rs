use crate::utils;

use super::Particle;

#[derive(Default, Clone, Copy)]
pub struct ParticleBuilder {
    inner: Particle,
}

impl ParticleBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_hash(mut self, hash: &str) -> Self {
        self.inner.hash = utils::hash(hash);
        self
    }

    pub fn set_mass(mut self, mass: f64) -> Self {
        self.inner.mass = mass;
        self
    }

    pub fn set_radius(mut self, radius: f64) -> Self {
        self.inner.radius = radius;
        self
    }

    pub fn set_x(mut self, x: f64) -> Self {
        self.inner.position.0 = x;
        self
    }

    pub fn set_y(mut self, y: f64) -> Self {
        self.inner.position.1 = y;
        self
    }

    pub fn set_z(mut self, z: f64) -> Self {
        self.inner.position.2 = z;
        self
    }

    pub fn set_position(mut self, x: f64, y: f64, z: f64) -> Self {
        self.inner.position = (x, y, z);
        self
    }

    pub fn set_vx(mut self, vx: f64) -> Self {
        self.inner.velocity.0 = vx;
        self
    }

    pub fn set_vy(mut self, vy: f64) -> Self {
        self.inner.velocity.1 = vy;
        self
    }

    pub fn set_vz(mut self, vz: f64) -> Self {
        self.inner.velocity.2 = vz;
        self
    }

    pub fn set_velocity(mut self, vx: f64, vy: f64, vz: f64) -> Self {
        self.inner.velocity = (vx, vy, vz);
        self
    }

    // TODO: Add other format support

    pub fn build(self) -> Particle {
        self.inner
    }
}

#[macro_export]
macro_rules! create_particle {
    ($($field:ident : $value:expr),* $(,)?) => {{
        let mut builder = $crate::particles::ParticleBuilder::new();
        $(
            builder = $crate::create_particle!(@set builder, $field, $value);
        )*
        builder.build()
    }};

    (@set $builder:ident, hash, $value:expr) => {
        $builder.set_hash($value)
    };
    (@set $builder:ident, mass, $value:expr) => {
        $builder.set_mass($value)
    };
    (@set $builder:ident, radius, $value:expr) => {
        $builder.set_radius($value)
    };
    (@set $builder:ident, x, $value:expr) => {
        $builder.set_x($value)
    };
    (@set $builder:ident, y, $value:expr) => {
        $builder.set_y($value)
    };
    (@set $builder:ident, z, $value:expr) => {
        $builder.set_z($value)
    };
    (@set $builder:ident, vx, $value:expr) => {
        $builder.set_vx($value)
    };
    (@set $builder:ident, vy, $value:expr) => {
        $builder.set_vy($value)
    };
    (@set $builder:ident, vz, $value:expr) => {
        $builder.set_vz($value)
    };
    (@set $builder:ident, $field:ident, $value:expr) => {
        compile_error!(concat!(
            "Unsupported field for create_particle!: ",
            stringify!($field),
            ". supported fields: hash, mass, radius, x, y, z, vx, vy, vz"
        ));
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn macro_test() {
        let p1 = create_particle! {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            vx: 0.1,
            vy: 0.2,
            vz: 0.3,
        };
        assert_eq!(p1.mass, 0.0);
        assert_eq!(p1.position, (1.0, 2.0, 3.0));
        assert_eq!(p1.velocity, (0.1, 0.2, 0.3));

        let p2 = create_particle! {
            mass: 1.0,
        };
        assert_eq!(p2.mass, 1.0);
        assert_eq!(p2.position, (0.0, 0.0, 0.0));
        assert_eq!(p2.velocity, (0.0, 0.0, 0.0));
    }
}

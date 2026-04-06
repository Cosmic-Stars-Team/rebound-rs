use crate::utils;

use super::Particle;

impl Particle {
    pub fn set_hash(mut self, hash: u32) -> Self {
        self.hash = hash;
        self
    }

    pub fn set_hash_by_name(self, hash: &str) -> Self {
        self.set_hash(utils::hash(hash))
    }

    pub fn set_mass(mut self, mass: f64) -> Self {
        self.mass = mass;
        self
    }

    pub fn set_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn set_x(mut self, x: f64) -> Self {
        self.position.0 = x;
        self
    }

    pub fn set_y(mut self, y: f64) -> Self {
        self.position.1 = y;
        self
    }

    pub fn set_z(mut self, z: f64) -> Self {
        self.position.2 = z;
        self
    }

    pub fn set_position(mut self, x: f64, y: f64, z: f64) -> Self {
        self.position = (x, y, z);
        self
    }

    pub fn set_vx(mut self, vx: f64) -> Self {
        self.velocity.0 = vx;
        self
    }

    pub fn set_vy(mut self, vy: f64) -> Self {
        self.velocity.1 = vy;
        self
    }

    pub fn set_vz(mut self, vz: f64) -> Self {
        self.velocity.2 = vz;
        self
    }

    pub fn set_velocity(mut self, vx: f64, vy: f64, vz: f64) -> Self {
        self.velocity = (vx, vy, vz);
        self
    }
}

#[allow(dead_code)]
#[doc(hidden)]
pub trait ParticleHashInput {
    fn apply_hash(self, particle: Particle) -> Particle;
}

impl ParticleHashInput for u32 {
    fn apply_hash(self, particle: Particle) -> Particle {
        particle.set_hash(self)
    }
}

impl ParticleHashInput for &str {
    fn apply_hash(self, particle: Particle) -> Particle {
        particle.set_hash_by_name(self)
    }
}

impl ParticleHashInput for String {
    fn apply_hash(self, particle: Particle) -> Particle {
        particle.set_hash_by_name(&self)
    }
}

impl ParticleHashInput for &String {
    fn apply_hash(self, particle: Particle) -> Particle {
        particle.set_hash_by_name(self)
    }
}

#[allow(dead_code)]
#[doc(hidden)]
pub fn _set_particle_hash<T>(particle: Particle, value: T) -> Particle
where
    T: ParticleHashInput,
{
    value.apply_hash(particle)
}

#[macro_export]
macro_rules! create_particle {
    ($($field:ident : $value:expr),* $(,)?) => {{
        let mut particle = $crate::particles::Particle::new();
        $(
            particle = $crate::create_particle!(@set particle, $field, $value);
        )*
        particle
    }};

    (@set $particle:ident, hash, $value:expr) => {
        $crate::particles::_set_particle_hash($particle, $value)
    };
    (@set $particle:ident, mass, $value:expr) => {
        $particle.set_mass($value)
    };
    (@set $particle:ident, radius, $value:expr) => {
        $particle.set_radius($value)
    };
    (@set $particle:ident, x, $value:expr) => {
        $particle.set_x($value)
    };
    (@set $particle:ident, y, $value:expr) => {
        $particle.set_y($value)
    };
    (@set $particle:ident, z, $value:expr) => {
        $particle.set_z($value)
    };
    (@set $particle:ident, vx, $value:expr) => {
        $particle.set_vx($value)
    };
    (@set $particle:ident, vy, $value:expr) => {
        $particle.set_vy($value)
    };
    (@set $particle:ident, vz, $value:expr) => {
        $particle.set_vz($value)
    };
    (@set $particle:ident, $field:ident, $value:expr) => {
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

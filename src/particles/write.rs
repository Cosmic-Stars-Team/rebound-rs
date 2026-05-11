use crate::{
    particles::{Particle, ParticleRead, ParticleRef},
    types::{Rotation, Vec3d},
};

/// Shared mutable interface for particle-like values.
///
/// Owned [`Particle`] values mutate themselves directly. [`ParticleRef`] values mutate the live
/// particle stored inside a simulation.
pub trait ParticleWrite: ParticleRead {
    fn try_set_hash(&mut self, hash: u32) -> Option<()>;
    fn try_set_mass(&mut self, mass: f64) -> Option<()>;
    fn try_set_radius(&mut self, radius: f64) -> Option<()>;
    fn try_set_position_vec3d(&mut self, position: Vec3d) -> Option<()>;
    fn try_set_velocity_vec3d(&mut self, velocity: Vec3d) -> Option<()>;
    fn try_set_acceleration_vec3d(&mut self, acceleration: Vec3d) -> Option<()>;

    fn try_set_position(&mut self, x: f64, y: f64, z: f64) -> Option<()> {
        self.try_set_position_vec3d(Vec3d(x, y, z))
    }

    fn try_set_velocity(&mut self, vx: f64, vy: f64, vz: f64) -> Option<()> {
        self.try_set_velocity_vec3d(Vec3d(vx, vy, vz))
    }

    fn try_set_acceleration(&mut self, ax: f64, ay: f64, az: f64) -> Option<()> {
        self.try_set_acceleration_vec3d(Vec3d(ax, ay, az))
    }

    fn try_irotate(&mut self, rotation: Rotation) -> Option<()> {
        let mut position = self.position()?;
        position.irotate(rotation);
        self.try_set_position_vec3d(position)?;

        let mut velocity = self.velocity()?;
        velocity.irotate(rotation);
        self.try_set_velocity_vec3d(velocity)?;

        Some(())
    }
}

impl ParticleWrite for Particle {
    fn try_set_hash(&mut self, hash: u32) -> Option<()> {
        self.hash = hash;
        Some(())
    }

    fn try_set_mass(&mut self, mass: f64) -> Option<()> {
        self.mass = mass;
        Some(())
    }

    fn try_set_radius(&mut self, radius: f64) -> Option<()> {
        self.radius = radius;
        Some(())
    }

    fn try_set_position_vec3d(&mut self, position: Vec3d) -> Option<()> {
        self.position = position;
        Some(())
    }

    fn try_set_velocity_vec3d(&mut self, velocity: Vec3d) -> Option<()> {
        self.velocity = velocity;
        Some(())
    }

    fn try_set_acceleration_vec3d(&mut self, acceleration: Vec3d) -> Option<()> {
        self.acceleration = acceleration;
        Some(())
    }
}

impl<'a> ParticleWrite for ParticleRef<'a> {
    fn try_set_hash(&mut self, hash: u32) -> Option<()> {
        self.set_hash(hash)
    }

    fn try_set_mass(&mut self, mass: f64) -> Option<()> {
        self.set_mass(mass)
    }

    fn try_set_radius(&mut self, radius: f64) -> Option<()> {
        self.set_radius(radius)
    }

    fn try_set_position_vec3d(&mut self, position: Vec3d) -> Option<()> {
        self.set_position_vec3d(position)
    }

    fn try_set_velocity_vec3d(&mut self, velocity: Vec3d) -> Option<()> {
        self.set_velocity_vec3d(velocity)
    }

    fn try_set_acceleration_vec3d(&mut self, acceleration: Vec3d) -> Option<()> {
        self.set_acceleration_vec3d(acceleration)
    }
}

#[cfg(test)]
mod tests {
    use crate::{particles::ParticleWrite, types::Vec3d};

    #[test]
    fn particle_write_updates_owned_particle() {
        let mut particle = crate::particles::Particle::new();

        particle.try_set_hash(1).unwrap();
        particle.try_set_mass(2.0).unwrap();
        particle.try_set_radius(3.0).unwrap();
        particle.try_set_position(4.0, 5.0, 6.0).unwrap();
        particle
            .try_set_velocity_vec3d(Vec3d(7.0, 8.0, 9.0))
            .unwrap();
        particle
            .try_set_acceleration_vec3d(Vec3d(10.0, 11.0, 12.0))
            .unwrap();

        assert_eq!(particle.hash, 1);
        assert_eq!(particle.mass, 2.0);
        assert_eq!(particle.radius, 3.0);
        assert_eq!(particle.position, Vec3d(4.0, 5.0, 6.0));
        assert_eq!(particle.velocity, Vec3d(7.0, 8.0, 9.0));
        assert_eq!(particle.acceleration, Vec3d(10.0, 11.0, 12.0));
    }
}

use core::marker::PhantomData;

use rebound_bind as rb;

use crate::{
    particles::{Orbit, Particle},
    types::{Rotation, Vec3d},
    utils,
};

pub struct ParticleRef<'a> {
    pub(crate) inner: *mut rb::reb_particle,
    pub(crate) _marker: PhantomData<&'a ()>,
}

impl<'a> ParticleRef<'a> {
    pub fn hash(&self) -> Option<u32> {
        Some(self.particle()?.hash)
    }

    pub fn set_hash(&mut self, hash: u32) -> Option<()> {
        self.particle_mut()?.hash = hash;
        Some(())
    }

    pub fn set_hash_by_name(&mut self, hash: &str) -> Option<()> {
        self.set_hash(utils::hash(hash))
    }

    pub fn position(&self) -> Option<Vec3d> {
        let particle = self.particle()?;
        Some(Vec3d(particle.x, particle.y, particle.z))
    }

    pub fn set_x(&mut self, x: f64) -> Option<()> {
        self.particle_mut()?.x = x;
        Some(())
    }

    pub fn set_y(&mut self, y: f64) -> Option<()> {
        self.particle_mut()?.y = y;
        Some(())
    }

    pub fn set_z(&mut self, z: f64) -> Option<()> {
        self.particle_mut()?.z = z;
        Some(())
    }

    pub fn set_position(&mut self, x: f64, y: f64, z: f64) -> Option<()> {
        let particle = self.particle_mut()?;
        particle.x = x;
        particle.y = y;
        particle.z = z;
        Some(())
    }

    pub fn velocity(&self) -> Option<Vec3d> {
        let particle = self.particle()?;
        Some(Vec3d(particle.vx, particle.vy, particle.vz))
    }

    pub fn set_vx(&mut self, vx: f64) -> Option<()> {
        self.particle_mut()?.vx = vx;
        Some(())
    }

    pub fn set_vy(&mut self, vy: f64) -> Option<()> {
        self.particle_mut()?.vy = vy;
        Some(())
    }

    pub fn set_vz(&mut self, vz: f64) -> Option<()> {
        self.particle_mut()?.vz = vz;
        Some(())
    }

    pub fn set_velocity(&mut self, vx: f64, vy: f64, vz: f64) -> Option<()> {
        let particle = self.particle_mut()?;
        particle.vx = vx;
        particle.vy = vy;
        particle.vz = vz;
        Some(())
    }

    pub fn acceleration(&self) -> Option<Vec3d> {
        let particle = self.particle()?;
        Some(Vec3d(particle.ax, particle.ay, particle.az))
    }

    pub fn set_ax(&mut self, ax: f64) -> Option<()> {
        self.particle_mut()?.ax = ax;
        Some(())
    }

    pub fn set_ay(&mut self, ay: f64) -> Option<()> {
        self.particle_mut()?.ay = ay;
        Some(())
    }

    pub fn set_az(&mut self, az: f64) -> Option<()> {
        self.particle_mut()?.az = az;
        Some(())
    }

    pub fn set_acceleration(&mut self, ax: f64, ay: f64, az: f64) -> Option<()> {
        let particle = self.particle_mut()?;
        particle.ax = ax;
        particle.ay = ay;
        particle.az = az;
        Some(())
    }

    pub fn mass(&self) -> Option<f64> {
        Some(self.particle()?.m)
    }

    pub fn set_mass(&mut self, mass: f64) -> Option<()> {
        self.particle_mut()?.m = mass;
        Some(())
    }

    pub fn radius(&self) -> Option<f64> {
        Some(self.particle()?.r)
    }

    pub fn set_radius(&mut self, radius: f64) -> Option<()> {
        self.particle_mut()?.r = radius;
        Some(())
    }

    pub fn last_collision(&self) -> Option<f64> {
        Some(self.particle()?.last_collision)
    }

    pub fn is_null(&self) -> bool {
        self.inner.is_null()
    }

    pub fn into_orbit(&self, g: f64, primary: &ParticleRef) -> Option<Orbit> {
        // TODO: Use reb_orbit_from_particle_err
        let orbit =
            unsafe { rb::reb_orbit_from_particle(g, *self.particle()?, *primary.particle()?) };
        Some(orbit.into())
    }

    pub fn irotate(&mut self, rotation: Rotation) -> Option<()> {
        let mut pos = self.position()?;
        pos.irotate(rotation);
        self.set_position(pos.0, pos.1, pos.2);

        let mut vel = self.velocity()?;
        vel.irotate(rotation);
        self.set_velocity(vel.0, vel.1, vel.2);
        Some(())
    }

    fn particle(&self) -> Option<&rb::reb_particle> {
        // Convert nullable raw pointer to shared reference in one place.
        unsafe { self.inner.as_ref() }
    }

    fn particle_mut(&mut self) -> Option<&mut rb::reb_particle> {
        // Convert nullable raw pointer to mutable reference in one place.
        unsafe { self.inner.as_mut() }
    }
}

impl<'a> From<ParticleRef<'a>> for Particle {
    fn from(particle: ParticleRef<'a>) -> Self {
        unsafe { (*particle.inner).into() }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        create_particle,
        simulation::{Simulation, SimulationParticlesRead, SimulationParticlesWrite},
        types::Vec3d,
    };

    #[test]
    fn setters_update_particle_fields() {
        let mut sim = Simulation::new();
        sim.add_particle(create_particle! {
            mass: 1.0,
            x: 1.0,
            y: 2.0,
            z: 3.0,
            vx: 4.0,
            vy: 5.0,
            vz: 6.0,
        })
        .unwrap();

        {
            let mut particle = sim.get_particle(0).unwrap();
            particle.set_hash(42).unwrap();
            particle.set_mass(2.0).unwrap();
            particle.set_radius(0.1).unwrap();
            particle.set_position(7.0, 8.0, 9.0).unwrap();
            particle.set_velocity(1.1, 1.2, 1.3).unwrap();
            particle.set_ax(0.1).unwrap();
            particle.set_ay(0.2).unwrap();
            particle.set_az(0.3).unwrap();
            particle.set_acceleration(2.1, 2.2, 2.3).unwrap();
        }

        let particle = sim.get_particle(0).unwrap();
        assert_eq!(particle.hash(), Some(42));
        assert_eq!(particle.mass(), Some(2.0));
        assert_eq!(particle.radius(), Some(0.1));
        assert_eq!(particle.position(), Some(Vec3d(7.0, 8.0, 9.0)));
        assert_eq!(particle.velocity(), Some(Vec3d(1.1, 1.2, 1.3)));
        assert_eq!(particle.acceleration(), Some(Vec3d(2.1, 2.2, 2.3)));
    }
}

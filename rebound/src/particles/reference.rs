use rebound_bind as rb;

use crate::{
    particles::{Particle, ParticlePosition},
    simulator::Simulation,
    utils,
};

pub struct ParticleRef<'a> {
    pub(crate) inner: *mut rb::reb_particle,
    pub(crate) _sim: &'a Simulation,
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

    pub fn position(&self) -> Option<ParticlePosition> {
        let particle = self.particle()?;
        Some((particle.x, particle.y, particle.z))
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

    pub fn velocity(&self) -> Option<ParticlePosition> {
        let particle = self.particle()?;
        Some((particle.vx, particle.vy, particle.vz))
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

    pub fn acceleration(&self) -> Option<ParticlePosition> {
        let particle = self.particle()?;
        Some((particle.ax, particle.ay, particle.az))
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
    use crate::{create_particle, simulator::Simulation};

    #[test]
    fn setters_update_particle_fields() {
        let sim = Simulation::new()
            .add_particle(create_particle! {
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
        }

        let particle = sim.get_particle(0).unwrap();
        assert_eq!(particle.hash(), Some(42));
        assert_eq!(particle.mass(), Some(2.0));
        assert_eq!(particle.radius(), Some(0.1));
        assert_eq!(particle.position(), Some((7.0, 8.0, 9.0)));
        assert_eq!(particle.velocity(), Some((1.1, 1.2, 1.3)));
    }
}

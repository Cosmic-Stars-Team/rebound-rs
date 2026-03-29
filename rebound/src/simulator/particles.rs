use crate::particles::ParticleRef;

use super::Simulation;
use rebound_bind as rb;

impl Simulation {
    pub fn add_particle(self, particle: crate::particles::Particle) -> Self {
        unsafe {
            rb::reb_simulation_add(self.inner, particle.into());
        }
        self
    }

    pub fn get_particle(&self, index: usize) -> Option<ParticleRef<'_>> {
        unsafe {
            let len = (*self.inner).N as usize;
            if index >= len {
                return None;
            }

            let particle = (*self.inner).particles.add(index);
            Some(ParticleRef {
                inner: particle,
                _sim: self,
            })
        }
    }

    pub fn get_particle_by_hash(&self, hash: u32) -> Option<ParticleRef<'_>> {
        unsafe {
            let particle = rb::reb_simulation_particle_by_hash(self.inner, hash);
            if particle.is_null() {
                return None;
            }
            Some(ParticleRef {
                inner: particle,
                _sim: self,
            })
        }
    }
}

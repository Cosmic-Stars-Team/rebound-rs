use crate::{
    particles::{Particle, ParticleRef},
    utils,
};

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
        let len = unsafe { (*self.inner).N as usize };
        if index >= len {
            return None;
        }

        let particle = unsafe { (*self.inner).particles.add(index) };
        Some(ParticleRef {
            inner: particle,
            _sim: self,
        })
    }

    pub fn get_particle_by_hash(&self, hash: u32) -> Option<ParticleRef<'_>> {
        let particle = unsafe { rb::reb_simulation_particle_by_hash(self.inner, hash) };

        if particle.is_null() {
            return None;
        }

        Some(ParticleRef {
            inner: particle,
            _sim: self,
        })
    }

    pub fn get_particle_by_hash_name(&self, name: &str) -> Option<ParticleRef<'_>> {
        let hash = utils::hash(name);
        self.get_particle_by_hash(hash)
    }

    pub fn com(&self) -> Particle {
        let com = unsafe { rb::reb_simulation_com(self.inner) };
        com.into()
    }

    pub fn com_range(&self, first: i32, last: i32) -> Particle {
        let com = unsafe { rb::reb_simulation_com_range(self.inner, first, last) };
        com.into()
    }
}

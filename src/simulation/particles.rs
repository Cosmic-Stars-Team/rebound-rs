use core::marker::PhantomData;

use crate::{
    Result,
    particles::{IntoParticle, Particle, ParticleRef},
    types::Rotation,
    utils,
};

use super::{SimulationSettingsRead, SimulationStateRead, SimulationWrite};
use rebound_bind as rb;

pub trait SimulationParticlesRead: SimulationStateRead {
    fn particles(&self) -> impl Iterator<Item = ParticleRef<'_>> + '_ {
        let len = self.n();
        (0..len).filter_map(move |i| self.get_particle(i))
    }

    fn get_particle(&self, index: usize) -> Option<ParticleRef<'_>> {
        if index >= self.n() {
            return None;
        }

        let particle = unsafe { (*self.raw()).particles.add(index) };
        Some(ParticleRef {
            inner: particle,
            _marker: PhantomData,
        })
    }

    fn get_particle_by_hash(&self, hash: u32) -> Option<ParticleRef<'_>> {
        let particle = unsafe {
            rb::reb_simulation_particle_by_hash(self.raw() as *mut rb::reb_simulation, hash)
        };

        if particle.is_null() {
            return None;
        }

        Some(ParticleRef {
            inner: particle,
            _marker: PhantomData,
        })
    }

    fn get_particle_by_hash_name(&self, name: &str) -> Option<ParticleRef<'_>> {
        let hash = utils::hash(name);
        self.get_particle_by_hash(hash)
    }

    fn com(&self) -> Particle {
        let com = unsafe { rb::reb_simulation_com(self.raw() as *mut rb::reb_simulation) };
        com.into()
    }

    fn com_range(&self, first: i32, last: i32) -> Particle {
        let com = unsafe {
            rb::reb_simulation_com_range(self.raw() as *mut rb::reb_simulation, first, last)
        };
        com.into()
    }
}

pub trait SimulationParticlesWrite:
    SimulationParticlesRead + SimulationSettingsRead + SimulationStateRead + SimulationWrite
{
    fn add_particle(&mut self, particle: impl IntoParticle) -> Result<&mut Self> {
        let particle = particle.with_simulation_defaults(&*self).into_particle()?;
        unsafe {
            rb::reb_simulation_add(self.raw_mut(), particle.into());
        }
        Ok(self)
    }

    fn irotate(&mut self, rotation: Rotation) -> Option<&mut Self> {
        for mut particle in self.particles() {
            particle.irotate(rotation)?;
        }
        Some(self)
    }
}

impl<T: SimulationStateRead + ?Sized> SimulationParticlesRead for T {}
impl<
    T: SimulationParticlesRead
        + SimulationSettingsRead
        + SimulationStateRead
        + SimulationWrite
        + ?Sized,
> SimulationParticlesWrite for T
{
}

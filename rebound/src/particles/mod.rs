mod builder;

use rebound_bind as rb;

use crate::simulator::Simulation;

pub use builder::ParticleBuilder;

pub type ParticlePosition = (f64, f64, f64);

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Particle {
    pub hash: u32,
    pub mass: f64,
    pub position: ParticlePosition,
    pub velocity: ParticlePosition,
}

impl From<Particle> for rb::reb_particle {
    fn from(particle: Particle) -> Self {
        rebound_bind::reb_particle {
            hash: particle.hash,
            x: particle.position.0,
            y: particle.position.1,
            z: particle.position.2,
            vx: particle.velocity.0,
            vy: particle.velocity.1,
            vz: particle.velocity.2,
            ax: 0.0,
            ay: 0.0,
            az: 0.0,
            m: particle.mass,
            r: 0.0,
            last_collision: 0.0,
            c: std::ptr::null_mut(),
            ap: std::ptr::null_mut(),
            sim: std::ptr::null_mut(),
        }
    }
}

pub struct ParticleRef<'a> {
    pub(crate) inner: *mut rb::reb_particle,
    pub(crate) _sim: &'a Simulation,
}

impl<'a> ParticleRef<'a> {
    pub fn hash(&self) -> Option<u32> {
        unsafe {
            if self.inner.is_null() {
                return None;
            }

            Some((*self.inner).hash)
        }
    }

    pub fn position(&self) -> Option<ParticlePosition> {
        unsafe {
            if self.inner.is_null() {
                return None;
            }

            Some(((*self.inner).x, (*self.inner).y, (*self.inner).z))
        }
    }

    pub fn velocity(&self) -> Option<ParticlePosition> {
        unsafe {
            if self.inner.is_null() {
                return None;
            }

            Some(((*self.inner).vx, (*self.inner).vy, (*self.inner).vz))
        }
    }

    pub fn acceleration(&self) -> Option<ParticlePosition> {
        unsafe {
            if self.inner.is_null() {
                return None;
            }

            Some(((*self.inner).ax, (*self.inner).ay, (*self.inner).az))
        }
    }

    pub fn mass(&self) -> Option<f64> {
        unsafe {
            if self.inner.is_null() {
                return None;
            }

            Some((*self.inner).m)
        }
    }

    pub fn radius(&self) -> Option<f64> {
        unsafe {
            if self.inner.is_null() {
                return None;
            }

            Some((*self.inner).r)
        }
    }

    pub fn last_collision(&self) -> Option<f64> {
        unsafe {
            if self.inner.is_null() {
                return None;
            }

            Some((*self.inner).last_collision)
        }
    }
}

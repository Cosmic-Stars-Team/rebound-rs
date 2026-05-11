mod builder;
mod integrator;
mod orbit;
mod read;
mod reference;
mod write;

use rebound_bind as rb;

pub use orbit::Orbit;
pub use orbit::{ClassicalOrbitalElementsBuilder, PalOrbitalElementsBuilder};
pub use read::ParticleRead;
pub use reference::ParticleRef;
pub use write::ParticleWrite;

use crate::types::Vec3d;
use crate::{
    Result,
    simulation::{SimulationParticlesRead, SimulationSettingsRead, SimulationStateRead},
};

#[doc(hidden)]
pub use builder::_set_particle_hash;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Particle {
    pub hash: u32,
    pub mass: f64,
    pub radius: f64,
    pub position: Vec3d,
    pub velocity: Vec3d,
    pub acceleration: Vec3d,
}

impl Particle {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<rb::reb_particle> for Particle {
    fn from(particle: rb::reb_particle) -> Self {
        Particle {
            hash: particle.hash,
            mass: particle.m,
            radius: particle.r,
            position: Vec3d(particle.x, particle.y, particle.z),
            velocity: Vec3d(particle.vx, particle.vy, particle.vz),
            acceleration: Vec3d(particle.ax, particle.ay, particle.az),
        }
    }
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
            ax: particle.acceleration.0,
            ay: particle.acceleration.1,
            az: particle.acceleration.2,
            m: particle.mass,
            r: particle.radius,
            last_collision: 0.0,
            c: std::ptr::null_mut(),
            ap: std::ptr::null_mut(),
            sim: std::ptr::null_mut(),
        }
    }
}

pub trait IntoParticle {
    fn with_simulation_defaults<S>(self, simulation: &S) -> Self
    where
        S: SimulationParticlesRead + SimulationSettingsRead + SimulationStateRead + ?Sized;

    fn into_particle(self) -> Result<Particle>;
}

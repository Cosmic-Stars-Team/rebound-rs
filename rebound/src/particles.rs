mod builder;
mod orbit;
mod reference;

use rebound_bind as rb;

pub use orbit::ClassicalOrbitalElementsBuilder;
pub use reference::ParticleRef;

pub type ParticlePosition = (f64, f64, f64);

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Particle {
    pub hash: u32,
    pub mass: f64,
    pub radius: f64,
    pub position: ParticlePosition,
    pub velocity: ParticlePosition,
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
            position: (particle.x, particle.y, particle.z),
            velocity: (particle.vx, particle.vy, particle.vz),
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
            ax: 0.0,
            ay: 0.0,
            az: 0.0,
            m: particle.mass,
            r: particle.radius,
            last_collision: 0.0,
            c: std::ptr::null_mut(),
            ap: std::ptr::null_mut(),
            sim: std::ptr::null_mut(),
        }
    }
}

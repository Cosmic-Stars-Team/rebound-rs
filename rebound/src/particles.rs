mod builder;
mod reference;

use rebound_bind as rb;

pub use builder::ParticleBuilder;
pub use reference::ParticleRef;

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

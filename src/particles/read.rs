use rebound_bind as rb;

use crate::{particles::Particle, types::Vec3d};

/// Shared read-only interface for owned particles and live particle references.
pub trait ParticleRead {
    /// Returns an owned snapshot of the particle data.
    fn snapshot(&self) -> Option<Particle>;

    /// Returns a raw REBOUND particle value copied from this particle-like object.
    fn raw_particle(&self) -> Option<rb::reb_particle> {
        Some(self.snapshot()?.into())
    }

    fn hash(&self) -> Option<u32> {
        Some(self.snapshot()?.hash)
    }

    fn mass(&self) -> Option<f64> {
        Some(self.snapshot()?.mass)
    }

    fn radius(&self) -> Option<f64> {
        Some(self.snapshot()?.radius)
    }

    fn position(&self) -> Option<Vec3d> {
        Some(self.snapshot()?.position)
    }

    fn velocity(&self) -> Option<Vec3d> {
        Some(self.snapshot()?.velocity)
    }

    fn acceleration(&self) -> Option<Vec3d> {
        Some(self.snapshot()?.acceleration)
    }
}

impl ParticleRead for Particle {
    fn snapshot(&self) -> Option<Particle> {
        Some(*self)
    }
}

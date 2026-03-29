use rebound_bind as rb;

use crate::{particles::ParticlePosition, simulator::Simulation};

pub struct ParticleRef<'a> {
    pub(crate) inner: *mut rb::reb_particle,
    pub(crate) _sim: &'a Simulation,
}

impl<'a> ParticleRef<'a> {
    pub fn hash(&self) -> Option<u32> {
        if self.inner.is_null() {
            return None;
        }

        unsafe { Some((*self.inner).hash) }
    }

    pub fn position(&self) -> Option<ParticlePosition> {
        if self.inner.is_null() {
            return None;
        }

        unsafe { Some(((*self.inner).x, (*self.inner).y, (*self.inner).z)) }
    }

    pub fn velocity(&self) -> Option<ParticlePosition> {
        if self.inner.is_null() {
            return None;
        }

        unsafe { Some(((*self.inner).vx, (*self.inner).vy, (*self.inner).vz)) }
    }

    pub fn acceleration(&self) -> Option<ParticlePosition> {
        if self.inner.is_null() {
            return None;
        }

        unsafe { Some(((*self.inner).ax, (*self.inner).ay, (*self.inner).az)) }
    }

    pub fn mass(&self) -> Option<f64> {
        if self.inner.is_null() {
            return None;
        }

        unsafe { Some((*self.inner).m) }
    }

    pub fn radius(&self) -> Option<f64> {
        if self.inner.is_null() {
            return None;
        }

        unsafe { Some((*self.inner).r) }
    }

    pub fn last_collision(&self) -> Option<f64> {
        if self.inner.is_null() {
            return None;
        }

        unsafe { Some((*self.inner).last_collision) }
    }
}

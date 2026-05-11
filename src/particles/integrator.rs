use rebound_bind as rb;

use super::Particle;

impl Particle {
    pub fn whfast_kepler_solver(&mut self, mass: f64, dt: f64) -> &mut Self {
        let mut raw_particle: rb::reb_particle = (*self).into();
        unsafe {
            rb::reb_whfast_kepler_solver(core::ptr::null(), &mut raw_particle, mass, 0, dt);
        }
        *self = raw_particle.into();
        self
    }
}

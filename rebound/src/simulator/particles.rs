use super::Simulation;
use rebound_bind as rb;

impl Simulation {
    pub fn add(&mut self, particle: crate::particles::Particle) -> &mut Self {
        unsafe {
            rb::reb_simulation_add(self.inner, particle.into());
        }
        self
    }
}

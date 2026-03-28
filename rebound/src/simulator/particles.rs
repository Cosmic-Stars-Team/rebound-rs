use super::Simulation;
use rebound_bind as rb;

impl Simulation {
    pub fn add_particle(self, particle: crate::particles::Particle) -> Self {
        unsafe {
            rb::reb_simulation_add(self.inner, particle.into());
        }
        self
    }
}

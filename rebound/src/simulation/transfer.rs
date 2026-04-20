use crate::simulation::Simulation;
use rebound_bind as rb;

impl Simulation {
    pub fn move_to_com(self) -> Self {
        unsafe {
            rb::reb_simulation_move_to_com(self.inner);
        }
        self
    }

    pub fn move_to_hel(self) -> Self {
        unsafe {
            rb::reb_simulation_move_to_hel(self.inner);
        }
        self
    }
}

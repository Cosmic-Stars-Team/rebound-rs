use crate::simulator::Simulation;
use rebound_bind as rb;

impl Clone for Simulation {
    fn clone(&self) -> Self {
        unsafe {
            let inner = rb::reb_simulation_copy(self.inner);
            Simulation { inner }
        }
    }
}

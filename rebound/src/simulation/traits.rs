use crate::simulation::Simulation;
use rebound_bind as rb;

impl Clone for Simulation {
    fn clone(&self) -> Self {
        let cloned = Simulation::try_new().expect("failed to allocate REBOUND simulation");
        let mut warnings = rb::reb_simulation_binary_error_codes_REB_SIMULATION_BINARY_WARNING_NONE;

        unsafe {
            rb::reb_simulation_copy_with_messages(cloned.inner, self.inner, &mut warnings);
        }

        cloned
    }
}

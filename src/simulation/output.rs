use crate::simulation::SimulationWrite;
use rebound_bind as rb;

pub trait SimulationOutputWrite: SimulationWrite {
    fn output_timing(&mut self, tmax: f64) -> &mut Self {
        unsafe {
            rb::reb_simulation_output_timing(self.raw_mut(), tmax);
        }
        self
    }

    fn output_check(&mut self, t: f64) -> bool {
        unsafe { rb::reb_simulation_output_check(self.raw_mut(), t) == 1 }
    }
}

impl<T: SimulationWrite + ?Sized> SimulationOutputWrite for T {}

use crate::simulation::{_Simulation, Simulation};
use rebound_bind as rb;

pub trait SimulationRead {
    fn raw(&self) -> *const rb::reb_simulation;
}

pub trait SimulationWrite: SimulationRead {
    fn raw_mut(&mut self) -> *mut rb::reb_simulation;
}

pub(crate) trait SimulationReadInternal: SimulationRead {
    fn owned(&self) -> &_Simulation {
        unsafe { _Simulation::from_raw(self.raw()) }
    }
}

impl SimulationRead for Simulation {
    fn raw(&self) -> *const rb::reb_simulation {
        &raw const self._owned().raw
    }
}

impl SimulationWrite for Simulation {
    fn raw_mut(&mut self) -> *mut rb::reb_simulation {
        unsafe { &raw mut self._owned_mut().get_unchecked_mut().raw }
    }
}

impl<T: SimulationRead + ?Sized> SimulationReadInternal for T {}

impl Clone for Simulation {
    fn clone(&self) -> Self {
        let mut cloned = Simulation::try_new().expect("failed to allocate REBOUND simulation");
        let mut warnings = rb::reb_simulation_binary_error_codes_REB_SIMULATION_BINARY_WARNING_NONE;

        unsafe {
            rb::reb_simulation_copy_with_messages(
                cloned.raw_mut(),
                self.raw() as *mut rb::reb_simulation,
                &mut warnings,
            );
            super::clear_callback_trampolines(cloned.raw_mut());
        }

        cloned
    }
}

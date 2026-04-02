use rebound_bind as rb;

use crate::simulator::Simulation;

pub struct IntegratorWhfast512<'a> {
    pub(crate) inner: *mut rb::reb_integrator_whfast512,
    pub(crate) _sim: &'a Simulation,
}

impl<'a> IntegratorWhfast512<'a> {
    pub fn set_gr_potential(self, gr_potential: u32) -> Self {
        unsafe {
            (*self.inner).gr_potential = gr_potential;
        }
        self
    }

    pub fn set_n_systems(self, n_systems: u32) -> Self {
        unsafe {
            (*self.inner).N_systems = n_systems;
        }
        self
    }

    pub fn set_keep_unsynchronized(self, keep_unsynchronized: u32) -> Self {
        unsafe {
            (*self.inner).keep_unsynchronized = keep_unsynchronized;
        }
        self
    }

    pub fn gr_potential(&self) -> u32 {
        unsafe { (*self.inner).gr_potential }
    }

    pub fn n_systems(&self) -> u32 {
        unsafe { (*self.inner).N_systems }
    }

    pub fn keep_unsynchronized(&self) -> u32 {
        unsafe { (*self.inner).keep_unsynchronized }
    }
}

use rebound_bind as rb;

use crate::simulator::Simulation;

pub struct IntegratorLeapfrog<'a> {
    pub(crate) inner: *mut rb::reb_integrator_leapfrog,
    pub(crate) _sim: &'a Simulation,
}

impl<'a> IntegratorLeapfrog<'a> {
    pub fn set_order(self, order: u32) -> Self {
        unsafe {
            (*self.inner).order = order;
        }
        self
    }

    pub fn order(&self) -> u32 {
        unsafe { (*self.inner).order }
    }
}

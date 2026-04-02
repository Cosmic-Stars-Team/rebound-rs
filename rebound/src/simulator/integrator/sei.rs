use rebound_bind as rb;

use crate::simulator::Simulation;

pub struct IntegratorSei<'a> {
    pub(crate) inner: *mut rb::reb_integrator_sei,
    pub(crate) _sim: &'a Simulation,
}

impl<'a> IntegratorSei<'a> {
    pub fn set_omega(self, omega: f64) -> Self {
        unsafe {
            (*self.inner).OMEGA = omega;
        }
        self
    }

    pub fn set_omega_z(self, omega_z: f64) -> Self {
        unsafe {
            (*self.inner).OMEGAZ = omega_z;
        }
        self
    }

    pub fn omega(&self) -> f64 {
        unsafe { (*self.inner).OMEGA }
    }

    pub fn omega_z(&self) -> f64 {
        unsafe { (*self.inner).OMEGAZ }
    }
}

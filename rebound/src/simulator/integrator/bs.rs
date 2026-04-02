use rebound_bind as rb;

use crate::simulator::Simulation;

pub struct IntegratorBs<'a> {
    pub(crate) inner: *mut rb::reb_integrator_bs,
    pub(crate) _sim: &'a Simulation,
}

impl<'a> IntegratorBs<'a> {
    pub fn set_eps_abs(self, eps_abs: f64) -> Self {
        unsafe {
            (*self.inner).eps_abs = eps_abs;
        }
        self
    }

    pub fn set_eps_rel(self, eps_rel: f64) -> Self {
        unsafe {
            (*self.inner).eps_rel = eps_rel;
        }
        self
    }

    pub fn set_min_dt(self, min_dt: f64) -> Self {
        unsafe {
            (*self.inner).min_dt = min_dt;
        }
        self
    }

    pub fn set_max_dt(self, max_dt: f64) -> Self {
        unsafe {
            (*self.inner).max_dt = max_dt;
        }
        self
    }

    pub fn eps_abs(&self) -> f64 {
        unsafe { (*self.inner).eps_abs }
    }

    pub fn eps_rel(&self) -> f64 {
        unsafe { (*self.inner).eps_rel }
    }

    pub fn min_dt(&self) -> f64 {
        unsafe { (*self.inner).min_dt }
    }

    pub fn max_dt(&self) -> f64 {
        unsafe { (*self.inner).max_dt }
    }
}

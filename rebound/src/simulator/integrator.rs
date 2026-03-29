use crate::error::{IntegrateError, Result};

use super::Simulation;
use rebound_bind as rb;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Integrator {
    Ias15 = rb::reb_simulation_REB_INTEGRATOR_IAS15,
    Whfast = rb::reb_simulation_REB_INTEGRATOR_WHFAST,
    Sei = rb::reb_simulation_REB_INTEGRATOR_SEI,
    Leapfrog = rb::reb_simulation_REB_INTEGRATOR_LEAPFROG,
    None = rb::reb_simulation_REB_INTEGRATOR_NONE,
    Janus = rb::reb_simulation_REB_INTEGRATOR_JANUS,
    Mercurius = rb::reb_simulation_REB_INTEGRATOR_MERCURIUS,
    Saba = rb::reb_simulation_REB_INTEGRATOR_SABA,
    Eos = rb::reb_simulation_REB_INTEGRATOR_EOS,
    Bs = rb::reb_simulation_REB_INTEGRATOR_BS,
    Whfast512 = rb::reb_simulation_REB_INTEGRATOR_WHFAST512,
    Trace = rb::reb_simulation_REB_INTEGRATOR_TRACE,
}

impl From<Integrator> for rb::reb_simulation__bindgen_ty_2 {
    fn from(value: Integrator) -> Self {
        value as Self
    }
}

impl Integrator {
    fn from_raw(value: rb::reb_simulation__bindgen_ty_2) -> Option<Self> {
        match value {
            rb::reb_simulation_REB_INTEGRATOR_IAS15 => Some(Self::Ias15),
            rb::reb_simulation_REB_INTEGRATOR_WHFAST => Some(Self::Whfast),
            rb::reb_simulation_REB_INTEGRATOR_SEI => Some(Self::Sei),
            rb::reb_simulation_REB_INTEGRATOR_LEAPFROG => Some(Self::Leapfrog),
            rb::reb_simulation_REB_INTEGRATOR_NONE => Some(Self::None),
            rb::reb_simulation_REB_INTEGRATOR_JANUS => Some(Self::Janus),
            rb::reb_simulation_REB_INTEGRATOR_MERCURIUS => Some(Self::Mercurius),
            rb::reb_simulation_REB_INTEGRATOR_SABA => Some(Self::Saba),
            rb::reb_simulation_REB_INTEGRATOR_EOS => Some(Self::Eos),
            rb::reb_simulation_REB_INTEGRATOR_BS => Some(Self::Bs),
            rb::reb_simulation_REB_INTEGRATOR_WHFAST512 => Some(Self::Whfast512),
            rb::reb_simulation_REB_INTEGRATOR_TRACE => Some(Self::Trace),
            _ => None,
        }
    }
}

impl Simulation {
    pub fn integrator(&self) -> Option<Integrator> {
        unsafe { Integrator::from_raw((*self.inner).integrator) }
    }

    pub fn set_integrator(self, integrator: Integrator) -> Self {
        unsafe {
            (*self.inner).integrator = integrator.into();
        }
        self
    }

    pub fn integrate(&mut self, tmax: f64) -> Result<()> {
        let status = unsafe { rb::reb_simulation_integrate(self.inner, tmax) };

        match IntegrateError::from_reb_status(status) {
            Some(err) => Err(err.into()),
            None => Ok(()),
        }
    }
}

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

impl Simulation {
    pub fn set_g(self, g: f64) -> Self {
        unsafe {
            (*self.inner).G = g;
        }
        self
    }

    pub fn set_integrator(self, integrator: Integrator) -> Self {
        unsafe {
            (*self.inner).integrator = integrator.into();
        }
        self
    }
}

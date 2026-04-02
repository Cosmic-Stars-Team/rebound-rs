pub mod mercurius;

use super::Simulation;
use crate::{
    error::{IntegrateError, Result},
    simulator::mercurius::IntegratorMercurius,
};

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

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ias15 => "Ias15",
            Self::Whfast => "Whfast",
            Self::Sei => "Sei",
            Self::Leapfrog => "Leapfrog",
            Self::None => "None",
            Self::Janus => "Janus",
            Self::Mercurius => "Mercurius",
            Self::Saba => "Saba",
            Self::Eos => "Eos",
            Self::Bs => "Bs",
            Self::Whfast512 => "Whfast512",
            Self::Trace => "Trace",
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

    pub fn reset_integrator(self) -> Self {
        unsafe {
            rb::reb_simulation_reset_integrator(self.inner);
        }

        self
    }

    pub fn synchronize(self) -> Self {
        unsafe {
            rb::reb_simulation_synchronize(self.inner);
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

    pub fn ri_mercurius(&mut self) -> mercurius::IntegratorMercurius<'_> {
        let ptr = unsafe { &raw mut (*self.inner).ri_mercurius };
        IntegratorMercurius {
            inner: ptr,
            _sim: self,
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! _reb_simulation_set_integrator_config {
    ($sim:expr, { $($field:ident : $value:expr),* $(,)? }) => {{
        let sim = &mut $sim;

        match sim.integrator() {
            Some($crate::simulator::Integrator::Mercurius) => {
                let mut cfg = sim.ri_mercurius();
                $(
                    cfg = $crate::_reb_simulation_set_integrator_config!(
                        @set_mercurius cfg, $field, $value
                    );
                )*
                let _ = cfg;

                ::core::result::Result::<(), $crate::error::Error>::Ok(())
            }
            _ => ::core::result::Result::<(), $crate::error::Error>::Err(
                $crate::error::IntegratorConfigError::UnknownIntegrator.into(),
            ),
        }
    }};

    // Mercurius Config
    (@set_mercurius $cfg:ident, r_crit_hill, $value:expr) => { $cfg.set_r_crit_hill($value) };
    (@set_mercurius $cfg:ident, recalculate_coordinates_this_timestep, $value:expr) => {
        $cfg.set_recalculate_coordinates_this_timestep($value)
    };
    (@set_mercurius $cfg:ident, recalculate_r_crit_this_timestep, $value:expr) => {
        $cfg.set_recalculate_r_crit_this_timestep($value)
    };
    (@set_mercurius $cfg:ident, safe_mode, $value:expr) => { $cfg.set_safe_mode($value) };
    (@set_mercurius $cfg:ident, l, $value:expr) => { $cfg.set_l($value) };
    (@set_mercurius $cfg:ident, $field:ident, $value:expr) => {
        compile_error!(concat!(
            "Unsupported Mercurius field: ",
            stringify!($field),
            ". supported fields: r_crit_hill, recalculate_coordinates_this_timestep, recalculate_r_crit_this_timestep, safe_mode, l"
        ));
    };
}

pub use _reb_simulation_set_integrator_config as set_integrator_config;

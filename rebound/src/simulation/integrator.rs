pub mod bs;
pub mod eos;
pub mod ias15;
pub mod janus;
pub mod leapfrog;
pub mod mercurius;
pub mod saba;
pub mod sei;
pub mod trace;
pub mod whfast;
pub mod whfast512;

use super::Simulation;
use crate::{
    error::{IntegrateError, Result},
    simulation::{
        bs::IntegratorBs, eos::IntegratorEos, ias15::IntegratorIas15, janus::IntegratorJanus,
        leapfrog::IntegratorLeapfrog, mercurius::IntegratorMercurius, saba::IntegratorSaba,
        sei::IntegratorSei, trace::IntegratorTrace, whfast::IntegratorWhfast,
        whfast512::IntegratorWhfast512,
    },
};

use rebound_bind as rb;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Integrator {
    Ias15 = rb::reb_simulation_REB_INTEGRATOR_IAS15 as isize,
    Whfast = rb::reb_simulation_REB_INTEGRATOR_WHFAST as isize,
    Sei = rb::reb_simulation_REB_INTEGRATOR_SEI as isize,
    Leapfrog = rb::reb_simulation_REB_INTEGRATOR_LEAPFROG as isize,
    None = rb::reb_simulation_REB_INTEGRATOR_NONE as isize,
    Janus = rb::reb_simulation_REB_INTEGRATOR_JANUS as isize,
    Mercurius = rb::reb_simulation_REB_INTEGRATOR_MERCURIUS as isize,
    Saba = rb::reb_simulation_REB_INTEGRATOR_SABA as isize,
    Eos = rb::reb_simulation_REB_INTEGRATOR_EOS as isize,
    Bs = rb::reb_simulation_REB_INTEGRATOR_BS as isize,
    Whfast512 = rb::reb_simulation_REB_INTEGRATOR_WHFAST512 as isize,
    Trace = rb::reb_simulation_REB_INTEGRATOR_TRACE as isize,
}

impl From<Integrator> for rb::reb_simulation__bindgen_ty_2 {
    fn from(value: Integrator) -> Self {
        match value {
            Integrator::Ias15 => rb::reb_simulation_REB_INTEGRATOR_IAS15,
            Integrator::Whfast => rb::reb_simulation_REB_INTEGRATOR_WHFAST,
            Integrator::Sei => rb::reb_simulation_REB_INTEGRATOR_SEI,
            Integrator::Leapfrog => rb::reb_simulation_REB_INTEGRATOR_LEAPFROG,
            Integrator::None => rb::reb_simulation_REB_INTEGRATOR_NONE,
            Integrator::Janus => rb::reb_simulation_REB_INTEGRATOR_JANUS,
            Integrator::Mercurius => rb::reb_simulation_REB_INTEGRATOR_MERCURIUS,
            Integrator::Saba => rb::reb_simulation_REB_INTEGRATOR_SABA,
            Integrator::Eos => rb::reb_simulation_REB_INTEGRATOR_EOS,
            Integrator::Bs => rb::reb_simulation_REB_INTEGRATOR_BS,
            Integrator::Whfast512 => rb::reb_simulation_REB_INTEGRATOR_WHFAST512,
            Integrator::Trace => rb::reb_simulation_REB_INTEGRATOR_TRACE,
        }
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

    pub fn ri_sei(&mut self) -> sei::IntegratorSei<'_> {
        let ptr = unsafe { &raw mut (*self.inner).ri_sei };
        IntegratorSei {
            inner: ptr,
            _sim: self,
        }
    }

    pub fn ri_leapfrog(&mut self) -> leapfrog::IntegratorLeapfrog<'_> {
        let ptr = unsafe { &raw mut (*self.inner).ri_leapfrog };
        IntegratorLeapfrog {
            inner: ptr,
            _sim: self,
        }
    }

    pub fn ri_bs(&mut self) -> bs::IntegratorBs<'_> {
        let ptr = unsafe { &raw mut (*self.inner).ri_bs };
        IntegratorBs {
            inner: ptr,
            _sim: self,
        }
    }

    pub fn ri_ias15(&mut self) -> ias15::IntegratorIas15<'_> {
        let ptr = unsafe { &raw mut (*self.inner).ri_ias15 };
        IntegratorIas15 {
            inner: ptr,
            _sim: self,
        }
    }

    pub fn ri_janus(&mut self) -> janus::IntegratorJanus<'_> {
        let ptr = unsafe { &raw mut (*self.inner).ri_janus };
        IntegratorJanus {
            inner: ptr,
            _sim: self,
        }
    }

    pub fn ri_whfast(&mut self) -> whfast::IntegratorWhfast<'_> {
        let ptr = unsafe { &raw mut (*self.inner).ri_whfast };
        IntegratorWhfast {
            inner: ptr,
            _sim: self,
        }
    }

    pub fn ri_saba(&mut self) -> saba::IntegratorSaba<'_> {
        let ptr = unsafe { &raw mut (*self.inner).ri_saba };
        IntegratorSaba {
            inner: ptr,
            _sim: self,
        }
    }

    pub fn ri_eos(&mut self) -> eos::IntegratorEos<'_> {
        let ptr = unsafe { &raw mut (*self.inner).ri_eos };
        IntegratorEos {
            inner: ptr,
            _sim: self,
        }
    }

    pub fn ri_trace(&mut self) -> trace::IntegratorTrace<'_> {
        let ptr = unsafe { &raw mut (*self.inner).ri_trace };
        IntegratorTrace {
            inner: ptr,
            _sim: self,
        }
    }

    pub fn ri_whfast512(&mut self) -> whfast512::IntegratorWhfast512<'_> {
        let ptr = unsafe { &raw mut (*self.inner).ri_whfast512 };
        IntegratorWhfast512 {
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

        $(
            $crate::_reb_simulation_set_integrator_config!(@validate_field $field);
        )*

        (|| -> ::core::result::Result<(), $crate::error::Error> {
            match sim.integrator() {
                Some($crate::simulation::Integrator::Ias15) => {
                    #[allow(unused_assignments)]
                    let mut cfg = sim.ri_ias15();
                    $(
                        cfg = $crate::_reb_simulation_set_integrator_config!(
                            @apply_ias15 cfg, $field, $value
                        )?;
                    )*
                    let _ = cfg;

                    ::core::result::Result::<(), $crate::error::Error>::Ok(())
                }
                Some($crate::simulation::Integrator::Whfast) => {
                    #[allow(unused_assignments)]
                    let mut cfg = sim.ri_whfast();
                    $(
                        cfg = $crate::_reb_simulation_set_integrator_config!(
                            @apply_whfast cfg, $field, $value
                        )?;
                    )*
                    let _ = cfg;

                    ::core::result::Result::<(), $crate::error::Error>::Ok(())
                }
                Some($crate::simulation::Integrator::Sei) => {
                    #[allow(unused_assignments)]
                    let mut cfg = sim.ri_sei();
                    $(
                        cfg = $crate::_reb_simulation_set_integrator_config!(
                            @apply_sei cfg, $field, $value
                        )?;
                    )*
                    let _ = cfg;

                    ::core::result::Result::<(), $crate::error::Error>::Ok(())
                }
                Some($crate::simulation::Integrator::Leapfrog) => {
                    #[allow(unused_assignments)]
                    let mut cfg = sim.ri_leapfrog();
                    $(
                        cfg = $crate::_reb_simulation_set_integrator_config!(
                            @apply_leapfrog cfg, $field, $value
                        )?;
                    )*
                    let _ = cfg;

                    ::core::result::Result::<(), $crate::error::Error>::Ok(())
                }
                Some($crate::simulation::Integrator::Janus) => {
                    #[allow(unused_assignments)]
                    let mut cfg = sim.ri_janus();
                    $(
                        cfg = $crate::_reb_simulation_set_integrator_config!(
                            @apply_janus cfg, $field, $value
                        )?;
                    )*
                    let _ = cfg;

                    ::core::result::Result::<(), $crate::error::Error>::Ok(())
                }
                Some($crate::simulation::Integrator::Mercurius) => {
                    #[allow(unused_assignments)]
                    let mut cfg = sim.ri_mercurius();
                    $(
                        cfg = $crate::_reb_simulation_set_integrator_config!(
                            @apply_mercurius cfg, $field, $value
                        )?;
                    )*
                    let _ = cfg;

                    ::core::result::Result::<(), $crate::error::Error>::Ok(())
                }
                Some($crate::simulation::Integrator::Saba) => {
                    #[allow(unused_assignments)]
                    let mut cfg = sim.ri_saba();
                    $(
                        cfg = $crate::_reb_simulation_set_integrator_config!(
                            @apply_saba cfg, $field, $value
                        )?;
                    )*
                    let _ = cfg;

                    ::core::result::Result::<(), $crate::error::Error>::Ok(())
                }
                Some($crate::simulation::Integrator::Eos) => {
                    #[allow(unused_assignments)]
                    let mut cfg = sim.ri_eos();
                    $(
                        cfg = $crate::_reb_simulation_set_integrator_config!(
                            @apply_eos cfg, $field, $value
                        )?;
                    )*
                    let _ = cfg;

                    ::core::result::Result::<(), $crate::error::Error>::Ok(())
                }
                Some($crate::simulation::Integrator::Bs) => {
                    #[allow(unused_assignments)]
                    let mut cfg = sim.ri_bs();
                    $(
                        cfg = $crate::_reb_simulation_set_integrator_config!(
                            @apply_bs cfg, $field, $value
                        )?;
                    )*
                    let _ = cfg;

                    ::core::result::Result::<(), $crate::error::Error>::Ok(())
                }
                Some($crate::simulation::Integrator::Whfast512) => {
                    #[allow(unused_assignments)]
                    let mut cfg = sim.ri_whfast512();
                    $(
                        cfg = $crate::_reb_simulation_set_integrator_config!(
                            @apply_whfast512 cfg, $field, $value
                        )?;
                    )*
                    let _ = cfg;

                    ::core::result::Result::<(), $crate::error::Error>::Ok(())
                }
                Some($crate::simulation::Integrator::Trace) => {
                    #[allow(unused_assignments)]
                    let mut cfg = sim.ri_trace();
                    $(
                        cfg = $crate::_reb_simulation_set_integrator_config!(
                            @apply_trace cfg, $field, $value
                        )?;
                    )*
                    let _ = cfg;

                    ::core::result::Result::<(), $crate::error::Error>::Ok(())
                }
                _ => ::core::result::Result::<(), $crate::error::Error>::Err(
                    $crate::error::IntegratorConfigError::UnknownIntegrator.into(),
                ),
            }
        })()
    }};

    (@unsupported $integrator:expr, $field:ident) => {
        ::core::result::Result::<_, $crate::error::Error>::Err(
            $crate::error::IntegratorConfigError::UnsupportedField {
                integrator: $integrator,
                field: stringify!($field),
            }
            .into(),
        )
    };

    (@validate_field epsilon) => {};
    (@validate_field min_dt) => {};
    (@validate_field adaptive_mode) => {};
    (@validate_field corrector) => {};
    (@validate_field corrector2) => {};
    (@validate_field kernel) => {};
    (@validate_field coordinates) => {};
    (@validate_field recalculate_coordinates_this_timestep) => {};
    (@validate_field safe_mode) => {};
    (@validate_field keep_unsynchronized) => {};
    (@validate_field omega) => {};
    (@validate_field omega_z) => {};
    (@validate_field order) => {};
    (@validate_field scale_pos) => {};
    (@validate_field scale_vel) => {};
    (@validate_field recalculate_integer_coordinates_this_timestep) => {};
    (@validate_field r_crit_hill) => {};
    (@validate_field recalculate_r_crit_this_timestep) => {};
    (@validate_field l) => {};
    (@validate_field type) => {};
    (@validate_field phi0) => {};
    (@validate_field phi1) => {};
    (@validate_field n) => {};
    (@validate_field eps_abs) => {};
    (@validate_field eps_rel) => {};
    (@validate_field max_dt) => {};
    (@validate_field gr_potential) => {};
    (@validate_field n_systems) => {};
    (@validate_field s) => {};
    (@validate_field s_peri) => {};
    (@validate_field peri_mode) => {};
    (@validate_field peri_crit_eta) => {};
    (@validate_field $field:ident) => {
        compile_error!(concat!(
            "Unsupported integrator config field: ",
            stringify!($field)
        ))
    };

    (@apply_ias15 $cfg:ident, epsilon, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_epsilon($value))
    };
    (@apply_ias15 $cfg:ident, min_dt, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_min_dt($value))
    };
    (@apply_ias15 $cfg:ident, adaptive_mode, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_adaptive_mode($value))
    };
    (@apply_ias15 $cfg:ident, $field:ident, $value:expr) => {
        $crate::_reb_simulation_set_integrator_config!(@unsupported "Ias15", $field)
    };

    (@apply_whfast $cfg:ident, corrector, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_corrector($value))
    };
    (@apply_whfast $cfg:ident, corrector2, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_corrector2($value))
    };
    (@apply_whfast $cfg:ident, kernel, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_kernel($value))
    };
    (@apply_whfast $cfg:ident, coordinates, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_coordinates($value))
    };
    (@apply_whfast $cfg:ident, recalculate_coordinates_this_timestep, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok(
            $cfg.set_recalculate_coordinates_this_timestep($value)
        )
    };
    (@apply_whfast $cfg:ident, safe_mode, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_safe_mode($value))
    };
    (@apply_whfast $cfg:ident, keep_unsynchronized, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok(
            $cfg.set_keep_unsynchronized($value)
        )
    };
    (@apply_whfast $cfg:ident, $field:ident, $value:expr) => {
        $crate::_reb_simulation_set_integrator_config!(@unsupported "Whfast", $field)
    };

    (@apply_sei $cfg:ident, omega, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_omega($value))
    };
    (@apply_sei $cfg:ident, omega_z, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_omega_z($value))
    };
    (@apply_sei $cfg:ident, $field:ident, $value:expr) => {
        $crate::_reb_simulation_set_integrator_config!(@unsupported "Sei", $field)
    };

    (@apply_leapfrog $cfg:ident, order, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_order($value))
    };
    (@apply_leapfrog $cfg:ident, $field:ident, $value:expr) => {
        $crate::_reb_simulation_set_integrator_config!(@unsupported "Leapfrog", $field)
    };

    (@apply_janus $cfg:ident, scale_pos, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_scale_pos($value))
    };
    (@apply_janus $cfg:ident, scale_vel, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_scale_vel($value))
    };
    (@apply_janus $cfg:ident, order, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_order($value))
    };
    (@apply_janus $cfg:ident, recalculate_integer_coordinates_this_timestep, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok(
            $cfg.set_recalculate_integer_coordinates_this_timestep($value)
        )
    };
    (@apply_janus $cfg:ident, $field:ident, $value:expr) => {
        $crate::_reb_simulation_set_integrator_config!(@unsupported "Janus", $field)
    };

    (@apply_mercurius $cfg:ident, r_crit_hill, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_r_crit_hill($value))
    };
    (@apply_mercurius $cfg:ident, recalculate_coordinates_this_timestep, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok(
            $cfg.set_recalculate_coordinates_this_timestep($value)
        )
    };
    (@apply_mercurius $cfg:ident, recalculate_r_crit_this_timestep, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok(
            $cfg.set_recalculate_r_crit_this_timestep($value)
        )
    };
    (@apply_mercurius $cfg:ident, safe_mode, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_safe_mode($value))
    };
    (@apply_mercurius $cfg:ident, l, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_l($value))
    };
    (@apply_mercurius $cfg:ident, $field:ident, $value:expr) => {
        $crate::_reb_simulation_set_integrator_config!(@unsupported "Mercurius", $field)
    };

    (@apply_saba $cfg:ident, type, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_type($value))
    };
    (@apply_saba $cfg:ident, safe_mode, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_safe_mode($value))
    };
    (@apply_saba $cfg:ident, keep_unsynchronized, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok(
            $cfg.set_keep_unsynchronized($value)
        )
    };
    (@apply_saba $cfg:ident, $field:ident, $value:expr) => {
        $crate::_reb_simulation_set_integrator_config!(@unsupported "Saba", $field)
    };

    (@apply_eos $cfg:ident, phi0, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_phi0($value))
    };
    (@apply_eos $cfg:ident, phi1, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_phi1($value))
    };
    (@apply_eos $cfg:ident, n, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_n($value))
    };
    (@apply_eos $cfg:ident, safe_mode, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_safe_mode($value))
    };
    (@apply_eos $cfg:ident, $field:ident, $value:expr) => {
        $crate::_reb_simulation_set_integrator_config!(@unsupported "Eos", $field)
    };

    (@apply_bs $cfg:ident, eps_abs, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_eps_abs($value))
    };
    (@apply_bs $cfg:ident, eps_rel, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_eps_rel($value))
    };
    (@apply_bs $cfg:ident, min_dt, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_min_dt($value))
    };
    (@apply_bs $cfg:ident, max_dt, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_max_dt($value))
    };
    (@apply_bs $cfg:ident, $field:ident, $value:expr) => {
        $crate::_reb_simulation_set_integrator_config!(@unsupported "Bs", $field)
    };

    (@apply_whfast512 $cfg:ident, gr_potential, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_gr_potential($value))
    };
    (@apply_whfast512 $cfg:ident, n_systems, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_n_systems($value))
    };
    (@apply_whfast512 $cfg:ident, keep_unsynchronized, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok(
            $cfg.set_keep_unsynchronized($value)
        )
    };
    (@apply_whfast512 $cfg:ident, $field:ident, $value:expr) => {
        $crate::_reb_simulation_set_integrator_config!(@unsupported "Whfast512", $field)
    };

    (@apply_trace $cfg:ident, s, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_s($value))
    };
    (@apply_trace $cfg:ident, s_peri, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_s_peri($value))
    };
    (@apply_trace $cfg:ident, peri_mode, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_peri_mode($value))
    };
    (@apply_trace $cfg:ident, r_crit_hill, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_r_crit_hill($value))
    };
    (@apply_trace $cfg:ident, peri_crit_eta, $value:expr) => {
        ::core::result::Result::<_, $crate::error::Error>::Ok($cfg.set_peri_crit_eta($value))
    };
    (@apply_trace $cfg:ident, $field:ident, $value:expr) => {
        $crate::_reb_simulation_set_integrator_config!(@unsupported "Trace", $field)
    };
}

pub use _reb_simulation_set_integrator_config as set_integrator_config;

#[cfg(test)]
mod tests {
    use super::{Integrator, Simulation, set_integrator_config};
    use crate::error::{Error, IntegratorConfigError};
    use rebound_bind as rb;

    #[test]
    fn integrator_roundtrip_uses_bindgen_raw_type() {
        let sim = Simulation::new().set_integrator(Integrator::Trace);

        assert_eq!(sim.integrator(), Some(Integrator::Trace));
        assert_eq!(
            rb::reb_simulation__bindgen_ty_2::from(Integrator::Trace),
            rb::reb_simulation_REB_INTEGRATOR_TRACE
        );
    }

    #[test]
    fn sei_direct_config_roundtrip() {
        let mut sim = Simulation::new().set_integrator(Integrator::Sei);
        let cfg = sim.ri_sei().set_omega(2.0).set_omega_z(3.0);

        assert_eq!(cfg.omega(), 2.0);
        assert_eq!(cfg.omega_z(), 3.0);
    }

    #[test]
    fn leapfrog_direct_config_roundtrip() {
        let mut sim = Simulation::new().set_integrator(Integrator::Leapfrog);
        let cfg = sim.ri_leapfrog().set_order(4);

        assert_eq!(cfg.order(), 4);
    }

    #[test]
    fn bs_direct_config_roundtrip() {
        let mut sim = Simulation::new().set_integrator(Integrator::Bs);
        let cfg = sim
            .ri_bs()
            .set_eps_abs(1e-9)
            .set_eps_rel(1e-10)
            .set_min_dt(0.01)
            .set_max_dt(0.1);

        assert_eq!(cfg.eps_abs(), 1e-9);
        assert_eq!(cfg.eps_rel(), 1e-10);
        assert_eq!(cfg.min_dt(), 0.01);
        assert_eq!(cfg.max_dt(), 0.1);
    }

    #[test]
    fn janus_direct_config_roundtrip() {
        let mut sim = Simulation::new().set_integrator(Integrator::Janus);
        let cfg = sim
            .ri_janus()
            .set_scale_pos(1e-15)
            .set_scale_vel(2e-15)
            .set_order(8)
            .set_recalculate_integer_coordinates_this_timestep(1);

        assert_eq!(cfg.scale_pos(), 1e-15);
        assert_eq!(cfg.scale_vel(), 2e-15);
        assert_eq!(cfg.order(), 8);
        assert_eq!(cfg.recalculate_integer_coordinates_this_timestep(), 1);
    }

    #[test]
    fn whfast512_direct_config_roundtrip() {
        let mut sim = Simulation::new().set_integrator(Integrator::Whfast512);
        let cfg = sim
            .ri_whfast512()
            .set_gr_potential(1)
            .set_n_systems(4)
            .set_keep_unsynchronized(1);

        assert_eq!(cfg.gr_potential(), 1);
        assert_eq!(cfg.n_systems(), 4);
        assert_eq!(cfg.keep_unsynchronized(), 1);
    }

    #[test]
    fn ias15_direct_config_roundtrip() {
        let mut sim = Simulation::new().set_integrator(Integrator::Ias15);
        let cfg = sim
            .ri_ias15()
            .set_epsilon(1e-12)
            .set_min_dt(1e-5)
            .set_adaptive_mode(crate::simulation::ias15::AdaptiveMode::Aarseth85);

        assert_eq!(cfg.epsilon(), 1e-12);
        assert_eq!(cfg.min_dt(), 1e-5);
        assert_eq!(
            cfg.adaptive_mode(),
            Some(crate::simulation::ias15::AdaptiveMode::Aarseth85)
        );
    }

    #[test]
    fn whfast_direct_config_roundtrip() {
        let mut sim = Simulation::new().set_integrator(Integrator::Whfast);
        let cfg = sim
            .ri_whfast()
            .set_corrector(17)
            .set_corrector2(1)
            .set_kernel(crate::simulation::whfast::Kernel::Lazy)
            .set_coordinates(crate::simulation::whfast::Coordinates::Barycentric)
            .set_recalculate_coordinates_this_timestep(1)
            .set_safe_mode(0)
            .set_keep_unsynchronized(1);

        assert_eq!(cfg.corrector(), 17);
        assert_eq!(cfg.corrector2(), 1);
        assert_eq!(cfg.kernel(), Some(crate::simulation::whfast::Kernel::Lazy));
        assert_eq!(
            cfg.coordinates(),
            Some(crate::simulation::whfast::Coordinates::Barycentric)
        );
        assert_eq!(cfg.recalculate_coordinates_this_timestep(), 1);
        assert_eq!(cfg.safe_mode(), 0);
        assert_eq!(cfg.keep_unsynchronized(), 1);
    }

    #[test]
    fn saba_direct_config_roundtrip() {
        let mut sim = Simulation::new().set_integrator(Integrator::Saba);
        let cfg = sim
            .ri_saba()
            .set_type(crate::simulation::saba::Type::SabaCl4)
            .set_safe_mode(0)
            .set_keep_unsynchronized(1);

        assert_eq!(cfg.kind(), Some(crate::simulation::saba::Type::SabaCl4));
        assert_eq!(cfg.safe_mode(), 0);
        assert_eq!(cfg.keep_unsynchronized(), 1);
    }

    #[test]
    fn eos_direct_config_roundtrip() {
        let mut sim = Simulation::new().set_integrator(Integrator::Eos);
        let cfg = sim
            .ri_eos()
            .set_phi0(crate::simulation::eos::Type::Lf8)
            .set_phi1(crate::simulation::eos::Type::Pmlf6)
            .set_n(4)
            .set_safe_mode(0);

        assert_eq!(cfg.phi0(), Some(crate::simulation::eos::Type::Lf8));
        assert_eq!(cfg.phi1(), Some(crate::simulation::eos::Type::Pmlf6));
        assert_eq!(cfg.n(), 4);
        assert_eq!(cfg.safe_mode(), 0);
    }

    fn safe_s(_: &mut crate::bind::reb_simulation, _: u32, _: u32) -> i32 {
        7
    }

    fn safe_s_peri(_: &mut crate::bind::reb_simulation, _: u32) -> i32 {
        11
    }

    #[test]
    fn trace_direct_config_roundtrip() {
        let mut sim = Simulation::new().set_integrator(Integrator::Trace);
        let cfg = sim
            .ri_trace()
            .set_s(crate::simulation::trace::S::Custom(
                crate::simulation::trace::trace_s_c_fn!(safe_s),
            ))
            .set_s_peri(crate::simulation::trace::SPeri::Custom(
                crate::simulation::trace::trace_s_peri_c_fn!(safe_s_peri),
            ))
            .set_peri_mode(crate::simulation::trace::PeriMode::FullIas15)
            .set_r_crit_hill(5.0)
            .set_peri_crit_eta(2.5);

        assert_eq!(
            cfg.peri_mode(),
            Some(crate::simulation::trace::PeriMode::FullIas15)
        );
        assert_eq!(cfg.r_crit_hill(), 5.0);
        assert_eq!(cfg.peri_crit_eta(), 2.5);
    }

    #[test]
    fn macro_sets_each_integrator_config() {
        let mut ias15_sim = Simulation::new().set_integrator(Integrator::Ias15);
        set_integrator_config!(ias15_sim, {
            epsilon: 1e-12,
            min_dt: 1e-5,
            adaptive_mode: crate::simulation::ias15::AdaptiveMode::Prs23,
        })
        .unwrap();
        assert_eq!(
            ias15_sim.ri_ias15().adaptive_mode(),
            Some(crate::simulation::ias15::AdaptiveMode::Prs23)
        );

        let mut mercurius_sim = Simulation::new().set_integrator(Integrator::Mercurius);
        set_integrator_config!(mercurius_sim, {
            r_crit_hill: 4.0,
            safe_mode: 0,
            l: crate::simulation::mercurius::L::Infinity,
        })
        .unwrap();
        assert_eq!(mercurius_sim.ri_mercurius().r_crit_hill(), 4.0);

        let mut whfast_sim = Simulation::new().set_integrator(Integrator::Whfast);
        set_integrator_config!(whfast_sim, {
            corrector: 17,
            coordinates: crate::simulation::whfast::Coordinates::Barycentric,
            keep_unsynchronized: 1,
        })
        .unwrap();
        assert_eq!(
            whfast_sim.ri_whfast().coordinates(),
            Some(crate::simulation::whfast::Coordinates::Barycentric)
        );

        let mut saba_sim = Simulation::new().set_integrator(Integrator::Saba);
        set_integrator_config!(saba_sim, {
            type: crate::simulation::saba::Type::Saba1064,
            safe_mode: 0,
        })
        .unwrap();
        assert_eq!(
            saba_sim.ri_saba().kind(),
            Some(crate::simulation::saba::Type::Saba1064)
        );

        let mut eos_sim = Simulation::new().set_integrator(Integrator::Eos);
        set_integrator_config!(eos_sim, {
            phi0: crate::simulation::eos::Type::Lf4,
            phi1: crate::simulation::eos::Type::Pmlf4,
            n: 3,
        })
        .unwrap();
        assert_eq!(
            eos_sim.ri_eos().phi1(),
            Some(crate::simulation::eos::Type::Pmlf4)
        );

        let mut trace_sim = Simulation::new().set_integrator(Integrator::Trace);
        set_integrator_config!(trace_sim, {
            peri_mode: crate::simulation::trace::PeriMode::FullBs,
            r_crit_hill: 6.0,
            peri_crit_eta: 1.5,
        })
        .unwrap();
        assert_eq!(
            trace_sim.ri_trace().peri_mode(),
            Some(crate::simulation::trace::PeriMode::FullBs)
        );
    }

    #[test]
    fn macro_returns_unknown_integrator_for_none() {
        let mut sim = Simulation::new().set_integrator(Integrator::None);

        let err = set_integrator_config!(sim, { safe_mode: 0 }).unwrap_err();

        assert_eq!(
            err,
            Error::IntegratorConfig(IntegratorConfigError::UnknownIntegrator)
        );
    }

    #[test]
    fn macro_returns_unsupported_field_for_wrong_integrator() {
        let mut sim = Simulation::new().set_integrator(Integrator::Sei);

        let err = set_integrator_config!(sim, { safe_mode: 0 }).unwrap_err();

        assert_eq!(
            err,
            Error::IntegratorConfig(IntegratorConfigError::UnsupportedField {
                integrator: "Sei",
                field: "safe_mode",
            })
        );
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! _reb_simulation_set_integrator_config {
    ($sim:expr, { $($field:ident : $value:expr),* $(,)? }) => {{
        let sim = &mut $sim;

        $(
            $crate::_reb_simulation_set_integrator_config!(@validate_field $field);
        )*

        (|| -> ::core::result::Result<(), $crate::error::Error> {
            match $crate::simulation::SimulationIntegratorRead::integrator(sim) {
                Some($crate::simulation::Integrator::Ias15) => {
                    #[allow(unused_assignments)]
                    let mut cfg = $crate::simulation::SimulationIntegratorWrite::ri_ias15(sim);
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
                    let mut cfg = $crate::simulation::SimulationIntegratorWrite::ri_whfast(sim);
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
                    let mut cfg = $crate::simulation::SimulationIntegratorWrite::ri_sei(sim);
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
                    let mut cfg = $crate::simulation::SimulationIntegratorWrite::ri_leapfrog(sim);
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
                    let mut cfg = $crate::simulation::SimulationIntegratorWrite::ri_janus(sim);
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
                    let mut cfg = $crate::simulation::SimulationIntegratorWrite::ri_mercurius(sim);
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
                    let mut cfg = $crate::simulation::SimulationIntegratorWrite::ri_saba(sim);
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
                    let mut cfg = $crate::simulation::SimulationIntegratorWrite::ri_eos(sim);
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
                    let mut cfg = $crate::simulation::SimulationIntegratorWrite::ri_bs(sim);
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
                    let mut cfg = $crate::simulation::SimulationIntegratorWrite::ri_whfast512(sim);
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
                    let mut cfg = $crate::simulation::SimulationIntegratorWrite::ri_trace(sim);
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

pub use crate::_reb_simulation_set_integrator_config as set_integrator_config;

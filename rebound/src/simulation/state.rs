use std::os::raw::c_int;

use crate::particles::ParticleRef;

use super::{SimulationRead, SimulationRef, SimulationRefMut};
use rebound_bind as rb;

pub type SimulationCallback = dyn for<'a> FnMut(SimulationRefMut<'a>) + 'static;
pub type SimulationKeyCallback = dyn for<'a> FnMut(SimulationRefMut<'a>, c_int) -> c_int + 'static;
pub type SimulationRestitutionCallback = dyn for<'a> FnMut(SimulationRef<'a>, f64) -> f64 + 'static;
pub type SimulationCollisionResolveCallback =
    dyn for<'a> FnMut(SimulationRefMut<'a>, rb::reb_collision) -> rb::REB_COLLISION_RESOLVE_OUTCOME
        + 'static;
pub type ParticleCallback = dyn for<'a> FnMut(ParticleRef<'a>) + 'static;

#[allow(dead_code)]
/// Rust-owned state associated with a pinned simulation.
///
/// REBOUND stores raw C function pointers in `reb_simulation`, so any safe Rust
/// callback that captures environment has to live outside the C struct. The
/// trampoline can then recover this state through the enclosing `_Simulation`.
#[derive(Default)]
pub struct SimulationState {
    pub(crate) additional_forces: Option<Box<SimulationCallback>>,
    pub(crate) pre_timestep_modifications: Option<Box<SimulationCallback>>,
    pub(crate) post_timestep_modifications: Option<Box<SimulationCallback>>,
    pub(crate) heartbeat: Option<Box<SimulationCallback>>,
    pub(crate) key_callback: Option<Box<SimulationKeyCallback>>,
    pub(crate) coefficient_of_restitution: Option<Box<SimulationRestitutionCallback>>,
    pub(crate) collision_resolve: Option<Box<SimulationCollisionResolveCallback>>,
    pub(crate) free_particle_ap: Option<Box<ParticleCallback>>,
    pub(crate) extras_cleanup: Option<Box<SimulationCallback>>,
}

impl SimulationState {
    pub const fn new() -> Self {
        Self {
            additional_forces: None,
            pre_timestep_modifications: None,
            post_timestep_modifications: None,
            heartbeat: None,
            key_callback: None,
            coefficient_of_restitution: None,
            collision_resolve: None,
            free_particle_ap: None,
            extras_cleanup: None,
        }
    }
}

pub trait SimulationStateRead: SimulationRead {
    fn t(&self) -> f64 {
        unsafe { (*self.raw()).t }
    }

    fn dt_last_done(&self) -> f64 {
        unsafe { (*self.raw()).dt_last_done }
    }

    fn steps_done(&self) -> u64 {
        unsafe { (*self.raw()).steps_done }
    }

    fn n(&self) -> usize {
        unsafe { (*self.raw()).N as usize }
    }

    fn n_var(&self) -> i32 {
        unsafe { (*self.raw()).N_var }
    }

    fn n_var_config(&self) -> usize {
        unsafe { (*self.raw()).N_var_config as usize }
    }

    fn status(&self) -> rb::REB_STATUS {
        unsafe { (*self.raw()).status }
    }

    fn output_timing_last(&self) -> f64 {
        unsafe { (*self.raw()).output_timing_last }
    }

    fn energy_offset(&self) -> f64 {
        unsafe { (*self.raw()).energy_offset }
    }

    fn walltime(&self) -> f64 {
        unsafe { (*self.raw()).walltime }
    }

    fn walltime_last_step(&self) -> f64 {
        unsafe { (*self.raw()).walltime_last_step }
    }

    fn walltime_last_steps(&self) -> f64 {
        unsafe { (*self.raw()).walltime_last_steps }
    }

    fn walltime_last_steps_sum(&self) -> f64 {
        unsafe { (*self.raw()).walltime_last_steps_sum }
    }

    fn walltime_last_steps_n(&self) -> i32 {
        unsafe { (*self.raw()).walltime_last_steps_N }
    }

    fn collisions_n(&self) -> usize {
        unsafe { (*self.raw()).collisions_N as usize }
    }

    fn collisions_log_n(&self) -> i64 {
        unsafe { (*self.raw()).collisions_log_n }
    }

    fn collisions_plog(&self) -> f64 {
        unsafe { (*self.raw()).collisions_plog }
    }
}

impl<T: SimulationRead + ?Sized> SimulationStateRead for T {}

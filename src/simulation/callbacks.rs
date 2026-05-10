use core::marker::PhantomData;
use std::os::raw::c_int;

use rebound_bind as rb;

use crate::{
    particles::ParticleRef,
    simulation::{SimulationRead, SimulationReadInternal},
};

use super::{
    _Simulation, ParticleCallback, SimulationCallback, SimulationCollisionResolveCallback,
    SimulationKeyCallback, SimulationRef, SimulationRefMut, SimulationRestitutionCallback,
    SimulationState, SimulationWrite,
};

type SimulationSlot = fn(&mut SimulationState) -> &mut Option<Box<SimulationCallback>>;
type SimulationKeySlot = fn(&mut SimulationState) -> &mut Option<Box<SimulationKeyCallback>>;
type SimulationRestitutionSlot =
    fn(&mut SimulationState) -> &mut Option<Box<SimulationRestitutionCallback>>;
type SimulationCollisionResolveSlot =
    fn(&mut SimulationState) -> &mut Option<Box<SimulationCollisionResolveCallback>>;
type ParticleSlot = fn(&mut SimulationState) -> &mut Option<Box<ParticleCallback>>;

fn additional_forces_slot(state: &mut SimulationState) -> &mut Option<Box<SimulationCallback>> {
    &mut state.additional_forces
}

fn pre_timestep_modifications_slot(
    state: &mut SimulationState,
) -> &mut Option<Box<SimulationCallback>> {
    &mut state.pre_timestep_modifications
}

fn post_timestep_modifications_slot(
    state: &mut SimulationState,
) -> &mut Option<Box<SimulationCallback>> {
    &mut state.post_timestep_modifications
}

fn heartbeat_slot(state: &mut SimulationState) -> &mut Option<Box<SimulationCallback>> {
    &mut state.heartbeat
}

fn key_callback_slot(state: &mut SimulationState) -> &mut Option<Box<SimulationKeyCallback>> {
    &mut state.key_callback
}

fn coefficient_of_restitution_slot(
    state: &mut SimulationState,
) -> &mut Option<Box<SimulationRestitutionCallback>> {
    &mut state.coefficient_of_restitution
}

fn collision_resolve_slot(
    state: &mut SimulationState,
) -> &mut Option<Box<SimulationCollisionResolveCallback>> {
    &mut state.collision_resolve
}

fn free_particle_ap_slot(state: &mut SimulationState) -> &mut Option<Box<ParticleCallback>> {
    &mut state.free_particle_ap
}

fn extras_cleanup_slot(state: &mut SimulationState) -> &mut Option<Box<SimulationCallback>> {
    &mut state.extras_cleanup
}

unsafe fn invoke_simulation_callback(raw: *mut rb::reb_simulation, slot: SimulationSlot) {
    let mut sim = unsafe { _Simulation::from_raw_mut(raw) };
    let callback = sim
        .as_ref()
        .get_ref()
        .with_state_mut(|state| slot(state).take());

    if let Some(mut callback) = callback {
        callback(sim.as_mut().sim_mut());
        sim.as_ref()
            .get_ref()
            .with_state_mut(|state| *slot(state) = Some(callback));
    }
}

unsafe fn invoke_key_callback(
    raw: *mut rb::reb_simulation,
    key: c_int,
    slot: SimulationKeySlot,
) -> c_int {
    let mut sim = unsafe { _Simulation::from_raw_mut(raw) };
    let callback = sim
        .as_ref()
        .get_ref()
        .with_state_mut(|state| slot(state).take());

    if let Some(mut callback) = callback {
        let result = callback(sim.as_mut().sim_mut(), key);
        sim.as_ref()
            .get_ref()
            .with_state_mut(|state| *slot(state) = Some(callback));
        result
    } else {
        0
    }
}

unsafe fn invoke_restitution_callback(
    raw: *const rb::reb_simulation,
    velocity: f64,
    slot: SimulationRestitutionSlot,
) -> f64 {
    let sim = unsafe { _Simulation::from_raw(raw) };
    let callback = sim.with_state_mut(|state| slot(state).take());

    if let Some(mut callback) = callback {
        let result = callback(sim.sim_ref(), velocity);
        sim.with_state_mut(|state| *slot(state) = Some(callback));
        result
    } else {
        1.0
    }
}

unsafe fn invoke_collision_resolve_callback(
    raw: *mut rb::reb_simulation,
    collision: rb::reb_collision,
    slot: SimulationCollisionResolveSlot,
) -> rb::REB_COLLISION_RESOLVE_OUTCOME {
    let mut sim = unsafe { _Simulation::from_raw_mut(raw) };
    let callback = sim
        .as_ref()
        .get_ref()
        .with_state_mut(|state| slot(state).take());

    if let Some(mut callback) = callback {
        let result = callback(sim.as_mut().sim_mut(), collision);
        sim.as_ref()
            .get_ref()
            .with_state_mut(|state| *slot(state) = Some(callback));
        result
    } else {
        unsafe { rb::reb_collision_resolve_halt(raw, collision) }
    }
}

unsafe fn invoke_particle_callback(particle: *mut rb::reb_particle, slot: ParticleSlot) {
    if particle.is_null() {
        return;
    }

    let raw = unsafe { (*particle).sim };
    if raw.is_null() {
        return;
    }

    let sim = unsafe { _Simulation::from_raw(raw) };
    let callback = sim.with_state_mut(|state| slot(state).take());

    if let Some(mut callback) = callback {
        callback(ParticleRef {
            inner: particle,
            _marker: PhantomData,
        });
        sim.with_state_mut(|state| *slot(state) = Some(callback));
    }
}

pub(crate) unsafe extern "C" fn additional_forces_trampoline(raw: *mut rb::reb_simulation) {
    unsafe { invoke_simulation_callback(raw, additional_forces_slot) }
}

pub(crate) unsafe extern "C" fn pre_timestep_modifications_trampoline(
    raw: *mut rb::reb_simulation,
) {
    unsafe { invoke_simulation_callback(raw, pre_timestep_modifications_slot) }
}

pub(crate) unsafe extern "C" fn post_timestep_modifications_trampoline(
    raw: *mut rb::reb_simulation,
) {
    unsafe { invoke_simulation_callback(raw, post_timestep_modifications_slot) }
}

pub(crate) unsafe extern "C" fn heartbeat_trampoline(raw: *mut rb::reb_simulation) {
    unsafe { invoke_simulation_callback(raw, heartbeat_slot) }
}

pub(crate) unsafe extern "C" fn key_callback_trampoline(
    raw: *mut rb::reb_simulation,
    key: c_int,
) -> c_int {
    unsafe { invoke_key_callback(raw, key, key_callback_slot) }
}

pub(crate) unsafe extern "C" fn coefficient_of_restitution_trampoline(
    raw: *const rb::reb_simulation,
    velocity: f64,
) -> f64 {
    unsafe { invoke_restitution_callback(raw, velocity, coefficient_of_restitution_slot) }
}

pub(crate) unsafe extern "C" fn collision_resolve_trampoline(
    raw: *mut rb::reb_simulation,
    collision: rb::reb_collision,
) -> rb::REB_COLLISION_RESOLVE_OUTCOME {
    unsafe { invoke_collision_resolve_callback(raw, collision, collision_resolve_slot) }
}

pub(crate) unsafe extern "C" fn free_particle_ap_trampoline(particle: *mut rb::reb_particle) {
    unsafe { invoke_particle_callback(particle, free_particle_ap_slot) }
}

pub(crate) unsafe extern "C" fn extras_cleanup_trampoline(raw: *mut rb::reb_simulation) {
    unsafe { invoke_simulation_callback(raw, extras_cleanup_slot) }
}

pub(crate) unsafe fn clear_callback_trampolines(raw: *mut rb::reb_simulation) {
    unsafe {
        (*raw).additional_forces = None;
        (*raw).pre_timestep_modifications = None;
        (*raw).post_timestep_modifications = None;
        (*raw).heartbeat = None;
        (*raw).key_callback = None;
        (*raw).coefficient_of_restitution = None;
        (*raw).collision_resolve = None;
        (*raw).free_particle_ap = None;
        (*raw).extras_cleanup = None;
    }
}

fn callback_state<S: SimulationReadInternal + ?Sized>(simulation: &S) -> &SimulationState {
    simulation.owned().state()
}

fn with_callback_state_mut<S: SimulationReadInternal + ?Sized, R>(
    simulation: &S,
    f: impl FnOnce(&mut SimulationState) -> R,
) -> R {
    simulation.owned().with_state_mut(f)
}

macro_rules! set_callback {
    ($self:expr, $state_field:ident, $raw_field:ident, $trampoline:ident, $callback:expr) => {{
        with_callback_state_mut($self, |state| {
            state.$state_field = Some(Box::new($callback));
        });
        unsafe {
            (*$self.raw_mut()).$raw_field = Some($trampoline);
        }
        $self
    }};
}

macro_rules! clear_callback {
    ($self:expr, $state_field:ident, $raw_field:ident) => {{
        with_callback_state_mut($self, |state| {
            state.$state_field = None;
        });
        unsafe {
            (*$self.raw_mut()).$raw_field = None;
        }
        $self
    }};
}

pub trait SimulationCallbacksRead: SimulationRead {
    fn has_additional_forces(&self) -> bool {
        callback_state(self).additional_forces.is_some()
    }

    fn has_pre_timestep_modifications(&self) -> bool {
        callback_state(self).pre_timestep_modifications.is_some()
    }

    fn has_post_timestep_modifications(&self) -> bool {
        callback_state(self).post_timestep_modifications.is_some()
    }

    fn has_heartbeat(&self) -> bool {
        callback_state(self).heartbeat.is_some()
    }

    fn has_key_callback(&self) -> bool {
        callback_state(self).key_callback.is_some()
    }

    fn has_coefficient_of_restitution(&self) -> bool {
        callback_state(self).coefficient_of_restitution.is_some()
    }

    fn has_collision_resolve(&self) -> bool {
        callback_state(self).collision_resolve.is_some()
    }

    fn has_free_particle_ap(&self) -> bool {
        callback_state(self).free_particle_ap.is_some()
    }

    fn has_extras_cleanup(&self) -> bool {
        callback_state(self).extras_cleanup.is_some()
    }
}

pub trait SimulationCallbacksWrite: SimulationCallbacksRead + SimulationWrite {
    fn set_additional_forces<F>(&mut self, callback: F) -> &mut Self
    where
        F: for<'a> FnMut(SimulationRefMut<'a>) + 'static,
    {
        set_callback!(
            self,
            additional_forces,
            additional_forces,
            additional_forces_trampoline,
            callback
        )
    }

    fn clear_additional_forces(&mut self) -> &mut Self {
        clear_callback!(self, additional_forces, additional_forces)
    }

    fn set_pre_timestep_modifications<F>(&mut self, callback: F) -> &mut Self
    where
        F: for<'a> FnMut(SimulationRefMut<'a>) + 'static,
    {
        set_callback!(
            self,
            pre_timestep_modifications,
            pre_timestep_modifications,
            pre_timestep_modifications_trampoline,
            callback
        )
    }

    fn clear_pre_timestep_modifications(&mut self) -> &mut Self {
        clear_callback!(self, pre_timestep_modifications, pre_timestep_modifications)
    }

    fn set_post_timestep_modifications<F>(&mut self, callback: F) -> &mut Self
    where
        F: for<'a> FnMut(SimulationRefMut<'a>) + 'static,
    {
        set_callback!(
            self,
            post_timestep_modifications,
            post_timestep_modifications,
            post_timestep_modifications_trampoline,
            callback
        )
    }

    fn clear_post_timestep_modifications(&mut self) -> &mut Self {
        clear_callback!(
            self,
            post_timestep_modifications,
            post_timestep_modifications
        )
    }

    fn set_heartbeat<F>(&mut self, callback: F) -> &mut Self
    where
        F: for<'a> FnMut(SimulationRefMut<'a>) + 'static,
    {
        set_callback!(self, heartbeat, heartbeat, heartbeat_trampoline, callback)
    }

    fn clear_heartbeat(&mut self) -> &mut Self {
        clear_callback!(self, heartbeat, heartbeat)
    }

    fn set_key_callback<F>(&mut self, callback: F) -> &mut Self
    where
        F: for<'a> FnMut(SimulationRefMut<'a>, c_int) -> c_int + 'static,
    {
        set_callback!(
            self,
            key_callback,
            key_callback,
            key_callback_trampoline,
            callback
        )
    }

    fn clear_key_callback(&mut self) -> &mut Self {
        clear_callback!(self, key_callback, key_callback)
    }

    fn set_coefficient_of_restitution<F>(&mut self, callback: F) -> &mut Self
    where
        F: for<'a> FnMut(SimulationRef<'a>, f64) -> f64 + 'static,
    {
        set_callback!(
            self,
            coefficient_of_restitution,
            coefficient_of_restitution,
            coefficient_of_restitution_trampoline,
            callback
        )
    }

    fn clear_coefficient_of_restitution(&mut self) -> &mut Self {
        clear_callback!(self, coefficient_of_restitution, coefficient_of_restitution)
    }

    fn set_collision_resolve<F>(&mut self, callback: F) -> &mut Self
    where
        F: for<'a> FnMut(
                SimulationRefMut<'a>,
                rb::reb_collision,
            ) -> rb::REB_COLLISION_RESOLVE_OUTCOME
            + 'static,
    {
        set_callback!(
            self,
            collision_resolve,
            collision_resolve,
            collision_resolve_trampoline,
            callback
        )
    }

    fn clear_collision_resolve(&mut self) -> &mut Self {
        clear_callback!(self, collision_resolve, collision_resolve)
    }

    fn set_free_particle_ap<F>(&mut self, callback: F) -> &mut Self
    where
        F: for<'a> FnMut(ParticleRef<'a>) + 'static,
    {
        set_callback!(
            self,
            free_particle_ap,
            free_particle_ap,
            free_particle_ap_trampoline,
            callback
        )
    }

    fn clear_free_particle_ap(&mut self) -> &mut Self {
        clear_callback!(self, free_particle_ap, free_particle_ap)
    }

    fn set_extras_cleanup<F>(&mut self, callback: F) -> &mut Self
    where
        F: for<'a> FnMut(SimulationRefMut<'a>) + 'static,
    {
        set_callback!(
            self,
            extras_cleanup,
            extras_cleanup,
            extras_cleanup_trampoline,
            callback
        )
    }

    fn clear_extras_cleanup(&mut self) -> &mut Self {
        clear_callback!(self, extras_cleanup, extras_cleanup)
    }
}

impl<T: SimulationRead + ?Sized> SimulationCallbacksRead for T {}
impl<T: SimulationWrite + ?Sized> SimulationCallbacksWrite for T {}

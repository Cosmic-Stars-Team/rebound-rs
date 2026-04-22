mod callbacks;
mod domain;
mod integrator;
mod particles;
mod reference;
mod settings;
mod state;
mod traits;
mod transfer;

pub use callbacks::*;
pub use domain::*;
pub use integrator::*;
pub use particles::*;
pub use reference::*;
pub use settings::*;
pub use state::*;
pub use traits::*;
pub use transfer::*;

use std::{
    alloc::{Layout, alloc},
    cell::UnsafeCell,
    marker::PhantomPinned,
    mem::MaybeUninit,
    pin::Pin,
};

use rebound_bind as rb;

use crate::{Error, Result};

pub struct Simulation {
    _owned: Pin<Box<_Simulation>>,
}

#[repr(C)]
pub(crate) struct _Simulation {
    // `raw` must remain the first field so callback trampolines can cast a
    // `*mut reb_simulation` back to `*mut _Simulation`.
    pub(crate) raw: rb::reb_simulation,
    state: UnsafeCell<state::SimulationState>,
    _pin: PhantomPinned,
}

impl _Simulation {
    fn new() -> Self {
        Self {
            // SAFETY: REBOUND initializes `reb_simulation` from a zeroed allocation.
            raw: unsafe { MaybeUninit::<rb::reb_simulation>::zeroed().assume_init() },
            state: UnsafeCell::new(state::SimulationState::new()),
            _pin: PhantomPinned,
        }
    }

    pub(crate) fn sim_ref(&self) -> SimulationRef<'_> {
        SimulationRef::new(self)
    }

    pub(crate) fn sim_mut(self: Pin<&mut Self>) -> SimulationRefMut<'_> {
        SimulationRefMut::new(self)
    }

    pub(crate) fn state(&self) -> &state::SimulationState {
        unsafe { &*self.state.get() }
    }

    pub(crate) fn with_state_mut<R>(&self, f: impl FnOnce(&mut state::SimulationState) -> R) -> R {
        unsafe { f(&mut *self.state.get()) }
    }

    #[allow(dead_code)]
    pub(crate) unsafe fn from_raw<'a>(raw: *const rb::reb_simulation) -> &'a Self {
        unsafe { &*raw.cast() }
    }

    #[allow(dead_code)]
    pub(crate) unsafe fn from_raw_mut<'a>(raw: *mut rb::reb_simulation) -> Pin<&'a mut Self> {
        unsafe { Pin::new_unchecked(&mut *raw.cast()) }
    }
}

impl Default for Simulation {
    fn default() -> Self {
        Self::try_new().expect("failed to allocate REBOUND simulation")
    }
}

impl Drop for Simulation {
    fn drop(&mut self) {
        let raw = unsafe { &raw mut self._owned_mut().get_unchecked_mut().raw };
        unsafe {
            rb::reb_simulation_free_pointers(raw);
        }
    }
}

impl Simulation {
    pub fn as_ref(&self) -> SimulationRef<'_> {
        self._owned().sim_ref()
    }

    pub fn as_mut(&mut self) -> SimulationRefMut<'_> {
        self._owned_mut().sim_mut()
    }

    pub(crate) fn _owned(&self) -> &_Simulation {
        self._owned.as_ref().get_ref()
    }

    pub(crate) fn _owned_mut(&mut self) -> Pin<&mut _Simulation> {
        self._owned.as_mut()
    }

    fn new_owned() -> Result<Pin<Box<_Simulation>>> {
        let layout = Layout::new::<_Simulation>();
        let ptr = unsafe { alloc(layout) as *mut _Simulation };
        if ptr.is_null() {
            return Err(Error::Allocation);
        }

        unsafe {
            ptr.write(_Simulation::new());
            Ok(Box::into_pin(Box::from_raw(ptr)))
        }
    }

    /// Create a new simulation.
    ///
    /// This constructor allocates the simulation backing storage in Rust,
    /// pins it to keep the embedded `reb_simulation` at a stable address,
    /// and initializes the raw REBOUND state in place.
    ///
    /// # Returns
    ///
    /// A new [`Simulation`] instance.
    ///
    /// # Panics
    ///
    /// Panics if the simulation could not be allocated. If you need to handle
    /// allocation errors, use [`Simulation::try_new`] instead.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new simulation.
    ///
    /// This constructor allocates the simulation backing storage in Rust,
    /// pins it to keep the embedded `reb_simulation` at a stable address,
    /// and initializes the raw REBOUND state in place.
    ///
    /// # Returns
    ///
    /// A new [`Simulation`] instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the simulation could not be allocated.
    pub fn try_new() -> Result<Self> {
        let mut owned = Self::new_owned()?;
        let raw = unsafe { &raw mut owned.as_mut().get_unchecked_mut().raw };

        unsafe {
            rb::reb_simulation_init(raw);
        }

        Ok(Self { _owned: owned })
    }
}

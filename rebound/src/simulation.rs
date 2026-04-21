mod domain;
mod integrator;
mod particles;
mod settings;
mod state;
mod traits;
mod transfer;

pub use integrator::*;
pub use settings::*;

use rebound_bind as rb;

use crate::{Error, Result};

pub struct Simulation {
    pub(crate) inner: *mut rb::reb_simulation,
}

impl Default for Simulation {
    fn default() -> Self {
        let inner = unsafe { rb::reb_simulation_create() };
        Self { inner }
    }
}

impl Drop for Simulation {
    fn drop(&mut self) {
        unsafe {
            rb::reb_simulation_free(self.inner);
        }
    }
}

impl Simulation {
    /// Create a new simulation.
    ///
    /// This is a convenience constructor that calls the underlying
    /// `rb::reb_simulation_create`. That function performs an allocation
    /// (using calloc) and then initializes the memory without checking
    /// whether the allocation returned NULL. Because of that, allocation
    /// failure in the upstream C code can lead to undefined behavior.
    ///
    /// If you need a fallible, safe constructor that returns an error on
    /// allocation failure, use `Simulation::try_new()` instead.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_new() -> Result<Self> {
        let ptr = unsafe { libc::calloc(1, std::mem::size_of::<rb::reb_simulation>()) }
            as *mut rb::reb_simulation;
        if ptr.is_null() {
            return Err(Error::Allocation);
        }

        unsafe {
            rb::reb_simulation_init(ptr);
        }

        Ok(Self { inner: ptr })
    }
}

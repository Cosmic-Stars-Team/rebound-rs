mod integrator;
mod options;
mod particles;
mod traits;
mod transfer;

pub use integrator::*;
pub use options::*;

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

    pub fn configure_box(self, boxsize: f64, n_root_x: i32, n_root_y: i32, n_root_z: i32) -> Self {
        unsafe {
            rb::reb_simulation_configure_box(self.inner, boxsize, n_root_x, n_root_y, n_root_z);
        }
        self
    }
}

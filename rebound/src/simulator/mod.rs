mod integrator;
mod options;
mod particles;
mod traits;

pub use options::Integrator;
use rebound_bind as rb;

pub struct Simulation {
    pub(crate) inner: *mut rb::reb_simulation,
}

impl Default for Simulation {
    fn default() -> Self {
        unsafe {
            let inner = rb::reb_simulation_create();
            Self { inner }
        }
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn configure_box(self, boxsize: f64, n_root_x: i32, n_root_y: i32, n_root_z: i32) -> Self {
        unsafe {
            rb::reb_simulation_configure_box(self.inner, boxsize, n_root_x, n_root_y, n_root_z);
        }
        self
    }
}

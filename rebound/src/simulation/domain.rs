use rebound_bind as rb;

use crate::{simulation::Simulation, types::Vec3d};

impl Simulation {
    pub fn configure_box(self, boxsize: f64, n_root_x: i32, n_root_y: i32, n_root_z: i32) -> Self {
        unsafe {
            rb::reb_simulation_configure_box(self.inner, boxsize, n_root_x, n_root_y, n_root_z);
        }
        self
    }

    pub fn boxsize(&self) -> Vec3d {
        unsafe { (*self.inner).boxsize.into() }
    }

    pub fn boxsize_max(&self) -> f64 {
        unsafe { (*self.inner).boxsize_max }
    }

    pub fn root_size(&self) -> f64 {
        unsafe { (*self.inner).root_size }
    }

    pub fn n_root(&self) -> i32 {
        unsafe { (*self.inner).N_root }
    }

    pub fn n_root_x(&self) -> i32 {
        unsafe { (*self.inner).N_root_x }
    }

    pub fn n_root_y(&self) -> i32 {
        unsafe { (*self.inner).N_root_y }
    }

    pub fn n_root_z(&self) -> i32 {
        unsafe { (*self.inner).N_root_z }
    }

    pub fn n_ghost_x(&self) -> i32 {
        unsafe { (*self.inner).N_ghost_x }
    }

    pub fn n_ghost_y(&self) -> i32 {
        unsafe { (*self.inner).N_ghost_y }
    }

    pub fn n_ghost_z(&self) -> i32 {
        unsafe { (*self.inner).N_ghost_z }
    }
}

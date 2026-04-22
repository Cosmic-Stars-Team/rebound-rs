use rebound_bind as rb;

use crate::types::Vec3d;

use super::{SimulationRead, SimulationWrite};

pub trait SimulationDomainWrite: SimulationWrite {
    fn configure_box(
        &mut self,
        boxsize: f64,
        n_root_x: i32,
        n_root_y: i32,
        n_root_z: i32,
    ) -> &mut Self {
        unsafe {
            rb::reb_simulation_configure_box(self.raw_mut(), boxsize, n_root_x, n_root_y, n_root_z);
        }
        self
    }
}

pub trait SimulationDomainRead: SimulationRead {
    fn boxsize(&self) -> Vec3d {
        unsafe { (*self.raw()).boxsize.into() }
    }

    fn boxsize_max(&self) -> f64 {
        unsafe { (*self.raw()).boxsize_max }
    }

    fn root_size(&self) -> f64 {
        unsafe { (*self.raw()).root_size }
    }

    fn n_root(&self) -> i32 {
        unsafe { (*self.raw()).N_root }
    }

    fn n_root_x(&self) -> i32 {
        unsafe { (*self.raw()).N_root_x }
    }

    fn n_root_y(&self) -> i32 {
        unsafe { (*self.raw()).N_root_y }
    }

    fn n_root_z(&self) -> i32 {
        unsafe { (*self.raw()).N_root_z }
    }

    fn n_ghost_x(&self) -> i32 {
        unsafe { (*self.raw()).N_ghost_x }
    }

    fn n_ghost_y(&self) -> i32 {
        unsafe { (*self.raw()).N_ghost_y }
    }

    fn n_ghost_z(&self) -> i32 {
        unsafe { (*self.raw()).N_ghost_z }
    }
}

impl<T: SimulationRead + ?Sized> SimulationDomainRead for T {}
impl<T: SimulationWrite + ?Sized> SimulationDomainWrite for T {}

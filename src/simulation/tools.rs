use crate::{
    simulation::{SimulationRead, SimulationWrite},
    types::Vec3d,
};
use rebound_bind as rb;

pub trait SimulationToolsWrite: SimulationWrite {
    fn energy(&mut self) -> f64 {
        unsafe { rb::reb_simulation_energy(self.raw_mut()) }
    }

    fn random_uniform(&mut self, min: f64, max: f64) -> f64 {
        unsafe { rb::reb_random_uniform(self.raw_mut(), min, max) }
    }

    fn random_powerlaw(&mut self, min: f64, max: f64, slope: f64) -> f64 {
        unsafe { rb::reb_random_powerlaw(self.raw_mut(), min, max, slope) }
    }

    fn random_normal(&mut self, variance: f64) -> f64 {
        unsafe { rb::reb_random_normal(self.raw_mut(), variance) }
    }

    fn random_rayleigh(&mut self, sigma: f64) -> f64 {
        unsafe { rb::reb_random_rayleigh(self.raw_mut(), sigma) }
    }
}

pub trait SimulationToolsRead: SimulationRead {
    fn angular_momentum(&self) -> Vec3d {
        unsafe { rb::reb_simulation_angular_momentum(self.raw()) }.into()
    }
}

impl<T: SimulationRead + ?Sized> SimulationToolsRead for T {}
impl<T: SimulationWrite + ?Sized> SimulationToolsWrite for T {}

use crate::{
    simulation::{SimulationRead, SimulationWrite},
    types::Vec3d,
};
use rebound_bind as rb;

pub trait SimulationToolsWrite: SimulationWrite {
    fn energy(&mut self) -> f64 {
        unsafe { rb::reb_simulation_energy(self.raw_mut()) }
    }
}

pub trait SimulationToolsRead: SimulationRead {
    fn angular_momentum(&self) -> Vec3d {
        unsafe { rb::reb_simulation_angular_momentum(self.raw()) }.into()
    }
}

impl<T: SimulationRead + ?Sized> SimulationToolsRead for T {}
impl<T: SimulationWrite + ?Sized> SimulationToolsWrite for T {}

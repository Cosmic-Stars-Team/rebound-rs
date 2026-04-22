use rebound_bind as rb;

use super::SimulationWrite;

pub trait SimulationTransferWrite: SimulationWrite {
    fn move_to_com(&mut self) -> &mut Self {
        unsafe {
            rb::reb_simulation_move_to_com(self.raw_mut());
        }
        self
    }

    fn move_to_hel(&mut self) -> &mut Self {
        unsafe {
            rb::reb_simulation_move_to_hel(self.raw_mut());
        }
        self
    }
}

impl<T: SimulationWrite + ?Sized> SimulationTransferWrite for T {}

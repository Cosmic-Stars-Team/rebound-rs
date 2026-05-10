use core::pin::Pin;

use rebound_bind as rb;

use super::{_Simulation, Simulation, SimulationRead, SimulationWrite};

pub struct SimulationRef<'a> {
    pub(crate) inner: &'a _Simulation,
}

impl<'a> SimulationRef<'a> {
    pub(crate) fn new(inner: &'a _Simulation) -> Self {
        Self { inner }
    }
}

impl<'a> From<&'a Simulation> for SimulationRef<'a> {
    fn from(simulation: &'a Simulation) -> Self {
        simulation.as_ref()
    }
}

impl<'a> SimulationRead for SimulationRef<'a> {
    fn raw(&self) -> *const rb::reb_simulation {
        &raw const self.inner.raw
    }
}

pub struct SimulationRefMut<'a> {
    pub(crate) inner: Pin<&'a mut _Simulation>,
}

impl<'a> SimulationRefMut<'a> {
    pub(crate) fn new(inner: Pin<&'a mut _Simulation>) -> Self {
        Self { inner }
    }

    pub fn as_ref(&self) -> SimulationRef<'_> {
        SimulationRef::new(self.inner.as_ref().get_ref())
    }

    pub fn as_mut(&mut self) -> SimulationRefMut<'_> {
        SimulationRefMut::new(self.inner.as_mut())
    }
}

impl<'a> From<&'a mut Simulation> for SimulationRefMut<'a> {
    fn from(simulation: &'a mut Simulation) -> Self {
        simulation.as_mut()
    }
}

impl<'a> SimulationRead for SimulationRefMut<'a> {
    fn raw(&self) -> *const rb::reb_simulation {
        &raw const self.inner.as_ref().get_ref().raw
    }
}

impl<'a> SimulationWrite for SimulationRefMut<'a> {
    fn raw_mut(&mut self) -> *mut rb::reb_simulation {
        unsafe { &raw mut self.inner.as_mut().get_unchecked_mut().raw }
    }
}

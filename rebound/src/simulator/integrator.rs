use crate::error::{IntegrateError, Result};

use super::Simulation;
use rebound_bind as rb;

impl Simulation {
    pub fn integrate(&mut self, tmax: f64) -> Result<()> {
        let status = unsafe { rb::reb_simulation_integrate(self.inner, tmax) };

        match IntegrateError::from_reb_status(status) {
            Some(err) => Err(err.into()),
            None => Ok(()),
        }
    }
}

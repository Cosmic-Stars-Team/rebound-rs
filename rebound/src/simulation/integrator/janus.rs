use rebound_bind as rb;

use crate::simulation::Simulation;

pub struct IntegratorJanus<'a> {
    pub(crate) inner: *mut rb::reb_integrator_janus,
    pub(crate) _sim: &'a Simulation,
}

impl<'a> IntegratorJanus<'a> {
    pub fn set_scale_pos(self, scale_pos: f64) -> Self {
        unsafe {
            (*self.inner).scale_pos = scale_pos;
        }
        self
    }

    pub fn set_scale_vel(self, scale_vel: f64) -> Self {
        unsafe {
            (*self.inner).scale_vel = scale_vel;
        }
        self
    }

    pub fn set_order(self, order: u32) -> Self {
        unsafe {
            (*self.inner).order = order;
        }
        self
    }

    pub fn set_recalculate_integer_coordinates_this_timestep(self, value: u32) -> Self {
        unsafe {
            (*self.inner).recalculate_integer_coordinates_this_timestep = value;
        }
        self
    }

    pub fn scale_pos(&self) -> f64 {
        unsafe { (*self.inner).scale_pos }
    }

    pub fn scale_vel(&self) -> f64 {
        unsafe { (*self.inner).scale_vel }
    }

    pub fn order(&self) -> u32 {
        unsafe { (*self.inner).order }
    }

    pub fn recalculate_integer_coordinates_this_timestep(&self) -> u32 {
        unsafe { (*self.inner).recalculate_integer_coordinates_this_timestep }
    }
}

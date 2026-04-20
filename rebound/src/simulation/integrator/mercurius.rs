use rebound_bind as rb;

use crate::simulation::Simulation;

pub enum L {
    Null,
    Mercurius,
    C4,
    C5,
    Infinity,
    Custom(unsafe extern "C" fn(r: *const rb::reb_simulation, d: f64, dcrit: f64) -> f64),
}

impl From<L>
    for Option<unsafe extern "C" fn(r: *const rb::reb_simulation, d: f64, dcrit: f64) -> f64>
{
    fn from(l: L) -> Self {
        match l {
            L::Null => None,
            L::Mercurius => Some(rb::reb_integrator_mercurius_L_mercury),
            L::C4 => Some(rb::reb_integrator_mercurius_L_C4),
            L::C5 => Some(rb::reb_integrator_mercurius_L_C5),
            L::Infinity => Some(rb::reb_integrator_mercurius_L_infinity),
            L::Custom(f) => Some(f),
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! _reb_integrator_mercurius_c_fn {
    ($safe_fn:path) => {{
        unsafe extern "C" fn wrapper(
            r: *const $crate::bind::reb_simulation,
            d: f64,
            dcrit: f64,
        ) -> f64 {
            // SAFETY: REBOUND invokes this callback with the active simulation pointer.
            let r_safe = unsafe { &mut *r };
            $safe_fn(r_safe, d, dcrit)
        }

        wrapper
    }};
}

pub use _reb_integrator_mercurius_c_fn as mercurius_c_fn;

pub struct IntegratorMercurius<'a> {
    pub(crate) inner: *mut rb::reb_integrator_mercurius,
    pub(crate) _sim: &'a Simulation,
}

impl<'a> IntegratorMercurius<'a> {
    pub fn set_r_crit_hill(self, r_crit_hill: f64) -> Self {
        unsafe {
            (*self.inner).r_crit_hill = r_crit_hill;
        }
        self
    }

    pub fn set_recalculate_coordinates_this_timestep(
        self,
        recalculate_coordinates_this_timestep: u32,
    ) -> Self {
        unsafe {
            (*self.inner).recalculate_coordinates_this_timestep =
                recalculate_coordinates_this_timestep;
        }
        self
    }

    pub fn set_recalculate_r_crit_this_timestep(
        self,
        recalculate_r_crit_this_timestep: u32,
    ) -> Self {
        unsafe {
            (*self.inner).recalculate_r_crit_this_timestep = recalculate_r_crit_this_timestep;
        }
        self
    }

    pub fn set_safe_mode(self, safe_mode: u32) -> Self {
        unsafe {
            (*self.inner).safe_mode = safe_mode;
        }
        self
    }

    pub fn set_l(self, l: L) -> Self {
        unsafe {
            (*self.inner).L = l.into();
        }
        self
    }

    pub fn r_crit_hill(&self) -> f64 {
        unsafe { (*self.inner).r_crit_hill }
    }

    pub fn recalculate_coordinates_this_timestep(&self) -> u32 {
        unsafe { (*self.inner).recalculate_coordinates_this_timestep }
    }

    pub fn recalculate_r_crit_this_timestep(&self) -> u32 {
        unsafe { (*self.inner).recalculate_r_crit_this_timestep }
    }

    pub fn safe_mode(&self) -> u32 {
        unsafe { (*self.inner).safe_mode }
    }
}

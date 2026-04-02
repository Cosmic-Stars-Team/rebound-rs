use rebound_bind as rb;

use crate::simulator::Simulation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Kernel {
    Default = rb::reb_integrator_whfast_REB_WHFAST_KERNEL_DEFAULT,
    ModifiedKick = rb::reb_integrator_whfast_REB_WHFAST_KERNEL_MODIFIEDKICK,
    Composition = rb::reb_integrator_whfast_REB_WHFAST_KERNEL_COMPOSITION,
    Lazy = rb::reb_integrator_whfast_REB_WHFAST_KERNEL_LAZY,
}

impl From<Kernel> for rb::reb_integrator_whfast__bindgen_ty_1 {
    fn from(value: Kernel) -> Self {
        value as Self
    }
}

impl Kernel {
    fn from_raw(value: rb::reb_integrator_whfast__bindgen_ty_1) -> Option<Self> {
        match value {
            rb::reb_integrator_whfast_REB_WHFAST_KERNEL_DEFAULT => Some(Self::Default),
            rb::reb_integrator_whfast_REB_WHFAST_KERNEL_MODIFIEDKICK => Some(Self::ModifiedKick),
            rb::reb_integrator_whfast_REB_WHFAST_KERNEL_COMPOSITION => Some(Self::Composition),
            rb::reb_integrator_whfast_REB_WHFAST_KERNEL_LAZY => Some(Self::Lazy),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Coordinates {
    Jacobi = rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_JACOBI,
    DemocraticHeliocentric =
        rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_DEMOCRATICHELIOCENTRIC,
    Whds = rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_WHDS,
    Barycentric = rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_BARYCENTRIC,
}

impl From<Coordinates> for rb::reb_integrator_whfast__bindgen_ty_2 {
    fn from(value: Coordinates) -> Self {
        value as Self
    }
}

impl Coordinates {
    fn from_raw(value: rb::reb_integrator_whfast__bindgen_ty_2) -> Option<Self> {
        match value {
            rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_JACOBI => Some(Self::Jacobi),
            rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_DEMOCRATICHELIOCENTRIC => {
                Some(Self::DemocraticHeliocentric)
            }
            rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_WHDS => Some(Self::Whds),
            rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_BARYCENTRIC => Some(Self::Barycentric),
            _ => None,
        }
    }
}

pub struct IntegratorWhfast<'a> {
    pub(crate) inner: *mut rb::reb_integrator_whfast,
    pub(crate) _sim: &'a Simulation,
}

impl<'a> IntegratorWhfast<'a> {
    pub fn set_corrector(self, corrector: u32) -> Self {
        unsafe {
            (*self.inner).corrector = corrector;
        }
        self
    }

    pub fn set_corrector2(self, corrector2: u32) -> Self {
        unsafe {
            (*self.inner).corrector2 = corrector2;
        }
        self
    }

    pub fn set_kernel(self, kernel: Kernel) -> Self {
        unsafe {
            (*self.inner).kernel = kernel.into();
        }
        self
    }

    pub fn set_coordinates(self, coordinates: Coordinates) -> Self {
        unsafe {
            (*self.inner).coordinates = coordinates.into();
        }
        self
    }

    pub fn set_recalculate_coordinates_this_timestep(self, value: u32) -> Self {
        unsafe {
            (*self.inner).recalculate_coordinates_this_timestep = value;
        }
        self
    }

    pub fn set_safe_mode(self, safe_mode: u32) -> Self {
        unsafe {
            (*self.inner).safe_mode = safe_mode;
        }
        self
    }

    pub fn set_keep_unsynchronized(self, keep_unsynchronized: u32) -> Self {
        unsafe {
            (*self.inner).keep_unsynchronized = keep_unsynchronized;
        }
        self
    }

    pub fn corrector(&self) -> u32 {
        unsafe { (*self.inner).corrector }
    }

    pub fn corrector2(&self) -> u32 {
        unsafe { (*self.inner).corrector2 }
    }

    pub fn kernel(&self) -> Option<Kernel> {
        unsafe { Kernel::from_raw((*self.inner).kernel) }
    }

    pub fn coordinates(&self) -> Option<Coordinates> {
        unsafe { Coordinates::from_raw((*self.inner).coordinates) }
    }

    pub fn recalculate_coordinates_this_timestep(&self) -> u32 {
        unsafe { (*self.inner).recalculate_coordinates_this_timestep }
    }

    pub fn safe_mode(&self) -> u32 {
        unsafe { (*self.inner).safe_mode }
    }

    pub fn keep_unsynchronized(&self) -> u32 {
        unsafe { (*self.inner).keep_unsynchronized }
    }
}

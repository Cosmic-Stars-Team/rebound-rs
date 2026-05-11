use crate::{
    error::{IntegratorConfigError, Result},
    particles::ParticleRef,
    simulation::{SimulationRead, SimulationWrite},
};
use rebound_bind as rb;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kernel {
    Default = rb::reb_integrator_whfast_REB_WHFAST_KERNEL_DEFAULT as isize,
    ModifiedKick = rb::reb_integrator_whfast_REB_WHFAST_KERNEL_MODIFIEDKICK as isize,
    Composition = rb::reb_integrator_whfast_REB_WHFAST_KERNEL_COMPOSITION as isize,
    Lazy = rb::reb_integrator_whfast_REB_WHFAST_KERNEL_LAZY as isize,
}

impl From<Kernel> for rb::reb_integrator_whfast__bindgen_ty_1 {
    fn from(value: Kernel) -> Self {
        match value {
            Kernel::Default => rb::reb_integrator_whfast_REB_WHFAST_KERNEL_DEFAULT,
            Kernel::ModifiedKick => rb::reb_integrator_whfast_REB_WHFAST_KERNEL_MODIFIEDKICK,
            Kernel::Composition => rb::reb_integrator_whfast_REB_WHFAST_KERNEL_COMPOSITION,
            Kernel::Lazy => rb::reb_integrator_whfast_REB_WHFAST_KERNEL_LAZY,
        }
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
pub enum Coordinates {
    Jacobi = rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_JACOBI as isize,
    DemocraticHeliocentric =
        rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_DEMOCRATICHELIOCENTRIC as isize,
    Whds = rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_WHDS as isize,
    Barycentric = rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_BARYCENTRIC as isize,
}

impl From<Coordinates> for rb::reb_integrator_whfast__bindgen_ty_2 {
    fn from(value: Coordinates) -> Self {
        match value {
            Coordinates::Jacobi => rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_JACOBI,
            Coordinates::DemocraticHeliocentric => {
                rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_DEMOCRATICHELIOCENTRIC
            }
            Coordinates::Whds => rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_WHDS,
            Coordinates::Barycentric => {
                rb::reb_integrator_whfast_REB_WHFAST_COORDINATES_BARYCENTRIC
            }
        }
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
    pub(crate) sim: *mut rb::reb_simulation,
    pub(crate) inner: *mut rb::reb_integrator_whfast,
    pub(crate) _marker: core::marker::PhantomData<&'a mut rb::reb_simulation>,
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

    pub fn from_inertial(&mut self) -> &mut Self {
        unsafe {
            rb::reb_integrator_whfast_from_inertial(self.sim);
        }
        self
    }

    pub fn to_inertial(&mut self) -> &mut Self {
        unsafe {
            rb::reb_integrator_whfast_to_inertial(self.sim);
        }
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe {
            rb::reb_integrator_whfast_reset(self.sim);
        }
        self
    }

    pub fn init(&mut self) -> Result<&mut Self> {
        let status = unsafe { rb::reb_integrator_whfast_init(self.sim) };
        if status == 0 {
            Ok(self)
        } else {
            Err(IntegratorConfigError::InitFailed {
                integrator: "Whfast",
            }
            .into())
        }
    }
}

pub trait SimulationIntegratorWhfastWrite: SimulationWrite {
    fn whfast_interaction_step(&mut self, dt: f64) -> &mut Self {
        unsafe {
            rb::reb_whfast_interaction_step(self.raw_mut(), dt);
        }
        self
    }

    fn whfast_jump_step(&mut self, dt: f64) -> &mut Self {
        unsafe {
            rb::reb_whfast_jump_step(self.raw_mut(), dt);
        }
        self
    }

    fn whfast_kepler_step(&mut self, dt: f64) -> &mut Self {
        unsafe {
            rb::reb_whfast_kepler_step(self.raw_mut(), dt);
        }
        self
    }

    fn whfast_com_step(&mut self, dt: f64) -> &mut Self {
        unsafe {
            rb::reb_whfast_com_step(self.raw_mut(), dt);
        }
        self
    }
}

pub trait SimulationIntegratorWhfastRead: SimulationRead {
    fn whfast_kepler_solver(
        &self,
        particle: &mut ParticleRef<'_>,
        mass: f64,
        dt: f64,
    ) -> Option<&Self> {
        if particle.is_null() {
            return None;
        }

        unsafe {
            rb::reb_whfast_kepler_solver(self.raw(), particle.inner, mass, 0, dt);
        }
        Some(self)
    }
}

impl<T: SimulationRead + ?Sized> SimulationIntegratorWhfastRead for T {}
impl<T: SimulationWrite + ?Sized> SimulationIntegratorWhfastWrite for T {}

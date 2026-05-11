mod r#macro;

pub mod bs;
pub mod eos;
pub mod ias15;
pub mod janus;
pub mod leapfrog;
pub mod mercurius;
pub mod saba;
pub mod sei;
pub mod trace;
pub mod whfast;
pub mod whfast512;

use core::marker::PhantomData;

use crate::{
    error::{IntegrateError, Result},
    simulation::{
        bs::IntegratorBs, eos::IntegratorEos, ias15::IntegratorIas15, janus::IntegratorJanus,
        leapfrog::IntegratorLeapfrog, mercurius::IntegratorMercurius, saba::IntegratorSaba,
        sei::IntegratorSei, trace::IntegratorTrace, whfast::IntegratorWhfast,
        whfast512::IntegratorWhfast512,
    },
};

use rebound_bind as rb;

use super::{SimulationRead, SimulationWrite};

pub use self::r#macro::set_integrator_config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Integrator {
    Ias15 = rb::reb_simulation_REB_INTEGRATOR_IAS15 as isize,
    Whfast = rb::reb_simulation_REB_INTEGRATOR_WHFAST as isize,
    Sei = rb::reb_simulation_REB_INTEGRATOR_SEI as isize,
    Leapfrog = rb::reb_simulation_REB_INTEGRATOR_LEAPFROG as isize,
    None = rb::reb_simulation_REB_INTEGRATOR_NONE as isize,
    Janus = rb::reb_simulation_REB_INTEGRATOR_JANUS as isize,
    Mercurius = rb::reb_simulation_REB_INTEGRATOR_MERCURIUS as isize,
    Saba = rb::reb_simulation_REB_INTEGRATOR_SABA as isize,
    Eos = rb::reb_simulation_REB_INTEGRATOR_EOS as isize,
    Bs = rb::reb_simulation_REB_INTEGRATOR_BS as isize,
    Whfast512 = rb::reb_simulation_REB_INTEGRATOR_WHFAST512 as isize,
    Trace = rb::reb_simulation_REB_INTEGRATOR_TRACE as isize,
}

impl From<Integrator> for rb::reb_simulation__bindgen_ty_2 {
    fn from(value: Integrator) -> Self {
        match value {
            Integrator::Ias15 => rb::reb_simulation_REB_INTEGRATOR_IAS15,
            Integrator::Whfast => rb::reb_simulation_REB_INTEGRATOR_WHFAST,
            Integrator::Sei => rb::reb_simulation_REB_INTEGRATOR_SEI,
            Integrator::Leapfrog => rb::reb_simulation_REB_INTEGRATOR_LEAPFROG,
            Integrator::None => rb::reb_simulation_REB_INTEGRATOR_NONE,
            Integrator::Janus => rb::reb_simulation_REB_INTEGRATOR_JANUS,
            Integrator::Mercurius => rb::reb_simulation_REB_INTEGRATOR_MERCURIUS,
            Integrator::Saba => rb::reb_simulation_REB_INTEGRATOR_SABA,
            Integrator::Eos => rb::reb_simulation_REB_INTEGRATOR_EOS,
            Integrator::Bs => rb::reb_simulation_REB_INTEGRATOR_BS,
            Integrator::Whfast512 => rb::reb_simulation_REB_INTEGRATOR_WHFAST512,
            Integrator::Trace => rb::reb_simulation_REB_INTEGRATOR_TRACE,
        }
    }
}

impl Integrator {
    fn from_raw(value: rb::reb_simulation__bindgen_ty_2) -> Option<Self> {
        match value {
            rb::reb_simulation_REB_INTEGRATOR_IAS15 => Some(Self::Ias15),
            rb::reb_simulation_REB_INTEGRATOR_WHFAST => Some(Self::Whfast),
            rb::reb_simulation_REB_INTEGRATOR_SEI => Some(Self::Sei),
            rb::reb_simulation_REB_INTEGRATOR_LEAPFROG => Some(Self::Leapfrog),
            rb::reb_simulation_REB_INTEGRATOR_NONE => Some(Self::None),
            rb::reb_simulation_REB_INTEGRATOR_JANUS => Some(Self::Janus),
            rb::reb_simulation_REB_INTEGRATOR_MERCURIUS => Some(Self::Mercurius),
            rb::reb_simulation_REB_INTEGRATOR_SABA => Some(Self::Saba),
            rb::reb_simulation_REB_INTEGRATOR_EOS => Some(Self::Eos),
            rb::reb_simulation_REB_INTEGRATOR_BS => Some(Self::Bs),
            rb::reb_simulation_REB_INTEGRATOR_WHFAST512 => Some(Self::Whfast512),
            rb::reb_simulation_REB_INTEGRATOR_TRACE => Some(Self::Trace),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ias15 => "Ias15",
            Self::Whfast => "Whfast",
            Self::Sei => "Sei",
            Self::Leapfrog => "Leapfrog",
            Self::None => "None",
            Self::Janus => "Janus",
            Self::Mercurius => "Mercurius",
            Self::Saba => "Saba",
            Self::Eos => "Eos",
            Self::Bs => "Bs",
            Self::Whfast512 => "Whfast512",
            Self::Trace => "Trace",
        }
    }
}

pub trait SimulationIntegratorRead: SimulationRead {
    fn integrator(&self) -> Option<Integrator> {
        unsafe { Integrator::from_raw((*self.raw()).integrator) }
    }
}

pub trait SimulationIntegratorWrite: SimulationIntegratorRead + SimulationWrite {
    fn set_integrator(&mut self, integrator: Integrator) -> &mut Self {
        unsafe {
            (*self.raw_mut()).integrator = integrator.into();
        }
        self
    }

    fn reset_integrator(&mut self) -> &mut Self {
        unsafe {
            rb::reb_simulation_reset_integrator(self.raw_mut());
        }

        self
    }

    fn synchronize(&mut self) -> &mut Self {
        unsafe {
            rb::reb_simulation_synchronize(self.raw_mut());
        }

        self
    }

    fn integrate(&mut self, tmax: f64) -> Result<&mut Self> {
        let status = unsafe { rb::reb_simulation_integrate(self.raw_mut(), tmax) };

        match IntegrateError::from_reb_status(status) {
            Some(err) => Err(err.into()),
            None => Ok(self),
        }
    }

    fn step(&mut self) -> &mut Self {
        unsafe { rb::reb_simulation_step(self.raw_mut()) };
        self
    }

    fn ri_mercurius(&mut self) -> mercurius::IntegratorMercurius<'_> {
        let ptr = unsafe { &raw mut (*self.raw_mut()).ri_mercurius };
        IntegratorMercurius {
            inner: ptr,
            _marker: PhantomData,
        }
    }

    fn ri_sei(&mut self) -> sei::IntegratorSei<'_> {
        let ptr = unsafe { &raw mut (*self.raw_mut()).ri_sei };
        IntegratorSei {
            inner: ptr,
            _marker: PhantomData,
        }
    }

    fn ri_leapfrog(&mut self) -> leapfrog::IntegratorLeapfrog<'_> {
        let ptr = unsafe { &raw mut (*self.raw_mut()).ri_leapfrog };
        IntegratorLeapfrog {
            inner: ptr,
            _marker: PhantomData,
        }
    }

    fn ri_bs(&mut self) -> bs::IntegratorBs<'_> {
        let ptr = unsafe { &raw mut (*self.raw_mut()).ri_bs };
        IntegratorBs {
            inner: ptr,
            _marker: PhantomData,
        }
    }

    fn ri_ias15(&mut self) -> ias15::IntegratorIas15<'_> {
        let sim = self.raw_mut();
        let ptr = unsafe { &raw mut (*sim).ri_ias15 };
        IntegratorIas15 {
            sim,
            inner: ptr,
            _marker: PhantomData,
        }
    }

    fn ri_janus(&mut self) -> janus::IntegratorJanus<'_> {
        let ptr = unsafe { &raw mut (*self.raw_mut()).ri_janus };
        IntegratorJanus {
            inner: ptr,
            _marker: PhantomData,
        }
    }

    fn ri_whfast(&mut self) -> whfast::IntegratorWhfast<'_> {
        let sim = self.raw_mut();
        let ptr = unsafe { &raw mut (*sim).ri_whfast };
        IntegratorWhfast {
            sim,
            inner: ptr,
            _marker: PhantomData,
        }
    }

    fn ri_saba(&mut self) -> saba::IntegratorSaba<'_> {
        let ptr = unsafe { &raw mut (*self.raw_mut()).ri_saba };
        IntegratorSaba {
            inner: ptr,
            _marker: PhantomData,
        }
    }

    fn ri_eos(&mut self) -> eos::IntegratorEos<'_> {
        let ptr = unsafe { &raw mut (*self.raw_mut()).ri_eos };
        IntegratorEos {
            inner: ptr,
            _marker: PhantomData,
        }
    }

    fn ri_trace(&mut self) -> trace::IntegratorTrace<'_> {
        let ptr = unsafe { &raw mut (*self.raw_mut()).ri_trace };
        IntegratorTrace {
            inner: ptr,
            _marker: PhantomData,
        }
    }

    fn ri_whfast512(&mut self) -> whfast512::IntegratorWhfast512<'_> {
        let ptr = unsafe { &raw mut (*self.raw_mut()).ri_whfast512 };
        IntegratorWhfast512 {
            inner: ptr,
            _marker: PhantomData,
        }
    }
}

impl<T: SimulationRead + ?Sized> SimulationIntegratorRead for T {}
impl<T: SimulationWrite + ?Sized> SimulationIntegratorWrite for T {}

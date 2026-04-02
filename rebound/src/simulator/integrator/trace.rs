use rebound_bind as rb;

use crate::simulator::Simulation;

pub enum S {
    Null,
    Default,
    Custom(unsafe extern "C" fn(r: *mut rb::reb_simulation, i: u32, j: u32) -> i32),
}

impl From<S> for Option<unsafe extern "C" fn(r: *mut rb::reb_simulation, i: u32, j: u32) -> i32> {
    fn from(value: S) -> Self {
        match value {
            S::Null => None,
            S::Default => Some(rb::reb_integrator_trace_switch_default),
            S::Custom(f) => Some(f),
        }
    }
}

pub enum SPeri {
    Null,
    Default,
    None,
    Custom(unsafe extern "C" fn(r: *mut rb::reb_simulation, j: u32) -> i32),
}

impl From<SPeri> for Option<unsafe extern "C" fn(r: *mut rb::reb_simulation, j: u32) -> i32> {
    fn from(value: SPeri) -> Self {
        match value {
            SPeri::Null => None,
            SPeri::Default => Some(rb::reb_integrator_trace_switch_peri_default),
            SPeri::None => Some(rb::reb_integrator_trace_switch_peri_none),
            SPeri::Custom(f) => Some(f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PeriMode {
    PartialBs = rb::reb_integrator_trace_REB_TRACE_PERI_PARTIAL_BS,
    FullBs = rb::reb_integrator_trace_REB_TRACE_PERI_FULL_BS,
    FullIas15 = rb::reb_integrator_trace_REB_TRACE_PERI_FULL_IAS15,
}

impl From<PeriMode> for rb::reb_integrator_trace__bindgen_ty_1 {
    fn from(value: PeriMode) -> Self {
        value as Self
    }
}

impl PeriMode {
    fn from_raw(value: rb::reb_integrator_trace__bindgen_ty_1) -> Option<Self> {
        match value {
            rb::reb_integrator_trace_REB_TRACE_PERI_PARTIAL_BS => Some(Self::PartialBs),
            rb::reb_integrator_trace_REB_TRACE_PERI_FULL_BS => Some(Self::FullBs),
            rb::reb_integrator_trace_REB_TRACE_PERI_FULL_IAS15 => Some(Self::FullIas15),
            _ => None,
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! _reb_integrator_trace_s_c_fn {
    ($safe_fn:path) => {{
        unsafe extern "C" fn wrapper(r: *mut $crate::bind::reb_simulation, i: u32, j: u32) -> i32 {
            let r_safe = unsafe { &mut *r };
            $safe_fn(r_safe, i, j)
        }

        wrapper
    }};
}

pub use _reb_integrator_trace_s_c_fn as trace_s_c_fn;

#[macro_export]
#[doc(hidden)]
macro_rules! _reb_integrator_trace_s_peri_c_fn {
    ($safe_fn:path) => {{
        unsafe extern "C" fn wrapper(r: *mut $crate::bind::reb_simulation, j: u32) -> i32 {
            let r_safe = unsafe { &mut *r };
            $safe_fn(r_safe, j)
        }

        wrapper
    }};
}

pub use _reb_integrator_trace_s_peri_c_fn as trace_s_peri_c_fn;

pub struct IntegratorTrace<'a> {
    pub(crate) inner: *mut rb::reb_integrator_trace,
    pub(crate) _sim: &'a Simulation,
}

impl<'a> IntegratorTrace<'a> {
    pub fn set_s(self, s: S) -> Self {
        unsafe {
            (*self.inner).S = s.into();
        }
        self
    }

    pub fn set_s_peri(self, s_peri: SPeri) -> Self {
        unsafe {
            (*self.inner).S_peri = s_peri.into();
        }
        self
    }

    pub fn set_peri_mode(self, peri_mode: PeriMode) -> Self {
        unsafe {
            (*self.inner).peri_mode = peri_mode.into();
        }
        self
    }

    pub fn set_r_crit_hill(self, r_crit_hill: f64) -> Self {
        unsafe {
            (*self.inner).r_crit_hill = r_crit_hill;
        }
        self
    }

    pub fn set_peri_crit_eta(self, peri_crit_eta: f64) -> Self {
        unsafe {
            (*self.inner).peri_crit_eta = peri_crit_eta;
        }
        self
    }

    pub fn peri_mode(&self) -> Option<PeriMode> {
        unsafe { PeriMode::from_raw((*self.inner).peri_mode) }
    }

    pub fn r_crit_hill(&self) -> f64 {
        unsafe { (*self.inner).r_crit_hill }
    }

    pub fn peri_crit_eta(&self) -> f64 {
        unsafe { (*self.inner).peri_crit_eta }
    }
}

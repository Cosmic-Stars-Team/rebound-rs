use rebound_bind as rb;

use crate::simulator::Simulation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Type {
    Wh = rb::reb_integrator_saba_REB_SABA_1,
    Saba2 = rb::reb_integrator_saba_REB_SABA_2,
    Saba3 = rb::reb_integrator_saba_REB_SABA_3,
    Saba4 = rb::reb_integrator_saba_REB_SABA_4,
    SabaCm1 = rb::reb_integrator_saba_REB_SABA_CM_1,
    SabaCm2 = rb::reb_integrator_saba_REB_SABA_CM_2,
    SabaCm3 = rb::reb_integrator_saba_REB_SABA_CM_3,
    SabaCm4 = rb::reb_integrator_saba_REB_SABA_CM_4,
    SabaCl1 = rb::reb_integrator_saba_REB_SABA_CL_1,
    SabaCl2 = rb::reb_integrator_saba_REB_SABA_CL_2,
    SabaCl3 = rb::reb_integrator_saba_REB_SABA_CL_3,
    SabaCl4 = rb::reb_integrator_saba_REB_SABA_CL_4,
    Saba104 = rb::reb_integrator_saba_REB_SABA_10_4,
    Saba864 = rb::reb_integrator_saba_REB_SABA_8_6_4,
    Saba1064 = rb::reb_integrator_saba_REB_SABA_10_6_4,
    SabaH844 = rb::reb_integrator_saba_REB_SABA_H_8_4_4,
    SabaH864 = rb::reb_integrator_saba_REB_SABA_H_8_6_4,
    SabaH1064 = rb::reb_integrator_saba_REB_SABA_H_10_6_4,
}

impl From<Type> for rb::reb_integrator_saba__bindgen_ty_1 {
    fn from(value: Type) -> Self {
        value as Self
    }
}

impl Type {
    fn from_raw(value: rb::reb_integrator_saba__bindgen_ty_1) -> Option<Self> {
        match value {
            rb::reb_integrator_saba_REB_SABA_1 => Some(Self::Wh),
            rb::reb_integrator_saba_REB_SABA_2 => Some(Self::Saba2),
            rb::reb_integrator_saba_REB_SABA_3 => Some(Self::Saba3),
            rb::reb_integrator_saba_REB_SABA_4 => Some(Self::Saba4),
            rb::reb_integrator_saba_REB_SABA_CM_1 => Some(Self::SabaCm1),
            rb::reb_integrator_saba_REB_SABA_CM_2 => Some(Self::SabaCm2),
            rb::reb_integrator_saba_REB_SABA_CM_3 => Some(Self::SabaCm3),
            rb::reb_integrator_saba_REB_SABA_CM_4 => Some(Self::SabaCm4),
            rb::reb_integrator_saba_REB_SABA_CL_1 => Some(Self::SabaCl1),
            rb::reb_integrator_saba_REB_SABA_CL_2 => Some(Self::SabaCl2),
            rb::reb_integrator_saba_REB_SABA_CL_3 => Some(Self::SabaCl3),
            rb::reb_integrator_saba_REB_SABA_CL_4 => Some(Self::SabaCl4),
            rb::reb_integrator_saba_REB_SABA_10_4 => Some(Self::Saba104),
            rb::reb_integrator_saba_REB_SABA_8_6_4 => Some(Self::Saba864),
            rb::reb_integrator_saba_REB_SABA_10_6_4 => Some(Self::Saba1064),
            rb::reb_integrator_saba_REB_SABA_H_8_4_4 => Some(Self::SabaH844),
            rb::reb_integrator_saba_REB_SABA_H_8_6_4 => Some(Self::SabaH864),
            rb::reb_integrator_saba_REB_SABA_H_10_6_4 => Some(Self::SabaH1064),
            _ => None,
        }
    }
}

pub struct IntegratorSaba<'a> {
    pub(crate) inner: *mut rb::reb_integrator_saba,
    pub(crate) _sim: &'a Simulation,
}

impl<'a> IntegratorSaba<'a> {
    pub fn set_type(self, type_: Type) -> Self {
        unsafe {
            (*self.inner).type_ = type_.into();
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

    pub fn kind(&self) -> Option<Type> {
        unsafe { Type::from_raw((*self.inner).type_) }
    }

    pub fn safe_mode(&self) -> u32 {
        unsafe { (*self.inner).safe_mode }
    }

    pub fn keep_unsynchronized(&self) -> u32 {
        unsafe { (*self.inner).keep_unsynchronized }
    }
}

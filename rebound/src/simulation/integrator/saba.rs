use rebound_bind as rb;

use crate::simulation::Simulation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Wh = rb::reb_integrator_saba_REB_SABA_1 as isize,
    Saba2 = rb::reb_integrator_saba_REB_SABA_2 as isize,
    Saba3 = rb::reb_integrator_saba_REB_SABA_3 as isize,
    Saba4 = rb::reb_integrator_saba_REB_SABA_4 as isize,
    SabaCm1 = rb::reb_integrator_saba_REB_SABA_CM_1 as isize,
    SabaCm2 = rb::reb_integrator_saba_REB_SABA_CM_2 as isize,
    SabaCm3 = rb::reb_integrator_saba_REB_SABA_CM_3 as isize,
    SabaCm4 = rb::reb_integrator_saba_REB_SABA_CM_4 as isize,
    SabaCl1 = rb::reb_integrator_saba_REB_SABA_CL_1 as isize,
    SabaCl2 = rb::reb_integrator_saba_REB_SABA_CL_2 as isize,
    SabaCl3 = rb::reb_integrator_saba_REB_SABA_CL_3 as isize,
    SabaCl4 = rb::reb_integrator_saba_REB_SABA_CL_4 as isize,
    Saba104 = rb::reb_integrator_saba_REB_SABA_10_4 as isize,
    Saba864 = rb::reb_integrator_saba_REB_SABA_8_6_4 as isize,
    Saba1064 = rb::reb_integrator_saba_REB_SABA_10_6_4 as isize,
    SabaH844 = rb::reb_integrator_saba_REB_SABA_H_8_4_4 as isize,
    SabaH864 = rb::reb_integrator_saba_REB_SABA_H_8_6_4 as isize,
    SabaH1064 = rb::reb_integrator_saba_REB_SABA_H_10_6_4 as isize,
}

impl From<Type> for rb::reb_integrator_saba__bindgen_ty_1 {
    fn from(value: Type) -> Self {
        match value {
            Type::Wh => rb::reb_integrator_saba_REB_SABA_1,
            Type::Saba2 => rb::reb_integrator_saba_REB_SABA_2,
            Type::Saba3 => rb::reb_integrator_saba_REB_SABA_3,
            Type::Saba4 => rb::reb_integrator_saba_REB_SABA_4,
            Type::SabaCm1 => rb::reb_integrator_saba_REB_SABA_CM_1,
            Type::SabaCm2 => rb::reb_integrator_saba_REB_SABA_CM_2,
            Type::SabaCm3 => rb::reb_integrator_saba_REB_SABA_CM_3,
            Type::SabaCm4 => rb::reb_integrator_saba_REB_SABA_CM_4,
            Type::SabaCl1 => rb::reb_integrator_saba_REB_SABA_CL_1,
            Type::SabaCl2 => rb::reb_integrator_saba_REB_SABA_CL_2,
            Type::SabaCl3 => rb::reb_integrator_saba_REB_SABA_CL_3,
            Type::SabaCl4 => rb::reb_integrator_saba_REB_SABA_CL_4,
            Type::Saba104 => rb::reb_integrator_saba_REB_SABA_10_4,
            Type::Saba864 => rb::reb_integrator_saba_REB_SABA_8_6_4,
            Type::Saba1064 => rb::reb_integrator_saba_REB_SABA_10_6_4,
            Type::SabaH844 => rb::reb_integrator_saba_REB_SABA_H_8_4_4,
            Type::SabaH864 => rb::reb_integrator_saba_REB_SABA_H_8_6_4,
            Type::SabaH1064 => rb::reb_integrator_saba_REB_SABA_H_10_6_4,
        }
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

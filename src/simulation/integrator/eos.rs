use rebound_bind as rb;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Lf = rb::REB_EOS_TYPE_REB_EOS_LF as isize,
    Lf4 = rb::REB_EOS_TYPE_REB_EOS_LF4 as isize,
    Lf6 = rb::REB_EOS_TYPE_REB_EOS_LF6 as isize,
    Lf8 = rb::REB_EOS_TYPE_REB_EOS_LF8 as isize,
    Lf42 = rb::REB_EOS_TYPE_REB_EOS_LF4_2 as isize,
    Lf864 = rb::REB_EOS_TYPE_REB_EOS_LF8_6_4 as isize,
    Plf764 = rb::REB_EOS_TYPE_REB_EOS_PLF7_6_4 as isize,
    Pmlf4 = rb::REB_EOS_TYPE_REB_EOS_PMLF4 as isize,
    Pmlf6 = rb::REB_EOS_TYPE_REB_EOS_PMLF6 as isize,
}

impl From<Type> for rb::REB_EOS_TYPE {
    fn from(value: Type) -> Self {
        match value {
            Type::Lf => rb::REB_EOS_TYPE_REB_EOS_LF,
            Type::Lf4 => rb::REB_EOS_TYPE_REB_EOS_LF4,
            Type::Lf6 => rb::REB_EOS_TYPE_REB_EOS_LF6,
            Type::Lf8 => rb::REB_EOS_TYPE_REB_EOS_LF8,
            Type::Lf42 => rb::REB_EOS_TYPE_REB_EOS_LF4_2,
            Type::Lf864 => rb::REB_EOS_TYPE_REB_EOS_LF8_6_4,
            Type::Plf764 => rb::REB_EOS_TYPE_REB_EOS_PLF7_6_4,
            Type::Pmlf4 => rb::REB_EOS_TYPE_REB_EOS_PMLF4,
            Type::Pmlf6 => rb::REB_EOS_TYPE_REB_EOS_PMLF6,
        }
    }
}

impl Type {
    fn from_raw(value: rb::REB_EOS_TYPE) -> Option<Self> {
        match value {
            rb::REB_EOS_TYPE_REB_EOS_LF => Some(Self::Lf),
            rb::REB_EOS_TYPE_REB_EOS_LF4 => Some(Self::Lf4),
            rb::REB_EOS_TYPE_REB_EOS_LF6 => Some(Self::Lf6),
            rb::REB_EOS_TYPE_REB_EOS_LF8 => Some(Self::Lf8),
            rb::REB_EOS_TYPE_REB_EOS_LF4_2 => Some(Self::Lf42),
            rb::REB_EOS_TYPE_REB_EOS_LF8_6_4 => Some(Self::Lf864),
            rb::REB_EOS_TYPE_REB_EOS_PLF7_6_4 => Some(Self::Plf764),
            rb::REB_EOS_TYPE_REB_EOS_PMLF4 => Some(Self::Pmlf4),
            rb::REB_EOS_TYPE_REB_EOS_PMLF6 => Some(Self::Pmlf6),
            _ => None,
        }
    }
}

pub struct IntegratorEos<'a> {
    pub(crate) inner: *mut rb::reb_integrator_eos,
    pub(crate) _marker: core::marker::PhantomData<&'a mut rb::reb_simulation>,
}

impl<'a> IntegratorEos<'a> {
    pub fn set_phi0(self, phi0: Type) -> Self {
        unsafe {
            (*self.inner).phi0 = phi0.into();
        }
        self
    }

    pub fn set_phi1(self, phi1: Type) -> Self {
        unsafe {
            (*self.inner).phi1 = phi1.into();
        }
        self
    }

    pub fn set_n(self, n: u32) -> Self {
        unsafe {
            (*self.inner).n = n;
        }
        self
    }

    pub fn set_safe_mode(self, safe_mode: u32) -> Self {
        unsafe {
            (*self.inner).safe_mode = safe_mode;
        }
        self
    }

    pub fn phi0(&self) -> Option<Type> {
        unsafe { Type::from_raw((*self.inner).phi0) }
    }

    pub fn phi1(&self) -> Option<Type> {
        unsafe { Type::from_raw((*self.inner).phi1) }
    }

    pub fn n(&self) -> u32 {
        unsafe { (*self.inner).n }
    }

    pub fn safe_mode(&self) -> u32 {
        unsafe { (*self.inner).safe_mode }
    }
}

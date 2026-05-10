use rebound_bind as rb;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdaptiveMode {
    Individual = rb::reb_integrator_ias15_REB_IAS15_INDIVIDUAL as isize,
    Global = rb::reb_integrator_ias15_REB_IAS15_GLOBAL as isize,
    Prs23 = rb::reb_integrator_ias15_REB_IAS15_PRS23 as isize,
    Aarseth85 = rb::reb_integrator_ias15_REB_IAS15_AARSETH85 as isize,
}

impl From<AdaptiveMode> for rb::reb_integrator_ias15__bindgen_ty_1 {
    fn from(value: AdaptiveMode) -> Self {
        match value {
            AdaptiveMode::Individual => rb::reb_integrator_ias15_REB_IAS15_INDIVIDUAL,
            AdaptiveMode::Global => rb::reb_integrator_ias15_REB_IAS15_GLOBAL,
            AdaptiveMode::Prs23 => rb::reb_integrator_ias15_REB_IAS15_PRS23,
            AdaptiveMode::Aarseth85 => rb::reb_integrator_ias15_REB_IAS15_AARSETH85,
        }
    }
}

impl AdaptiveMode {
    fn from_raw(value: rb::reb_integrator_ias15__bindgen_ty_1) -> Option<Self> {
        match value {
            rb::reb_integrator_ias15_REB_IAS15_INDIVIDUAL => Some(Self::Individual),
            rb::reb_integrator_ias15_REB_IAS15_GLOBAL => Some(Self::Global),
            rb::reb_integrator_ias15_REB_IAS15_PRS23 => Some(Self::Prs23),
            rb::reb_integrator_ias15_REB_IAS15_AARSETH85 => Some(Self::Aarseth85),
            _ => None,
        }
    }
}

pub struct IntegratorIas15<'a> {
    pub(crate) inner: *mut rb::reb_integrator_ias15,
    pub(crate) _marker: core::marker::PhantomData<&'a mut rb::reb_simulation>,
}

impl<'a> IntegratorIas15<'a> {
    pub fn set_epsilon(self, epsilon: f64) -> Self {
        unsafe {
            (*self.inner).epsilon = epsilon;
        }
        self
    }

    pub fn set_min_dt(self, min_dt: f64) -> Self {
        unsafe {
            (*self.inner).min_dt = min_dt;
        }
        self
    }

    pub fn set_adaptive_mode(self, adaptive_mode: AdaptiveMode) -> Self {
        unsafe {
            (*self.inner).adaptive_mode = adaptive_mode.into();
        }
        self
    }

    pub fn epsilon(&self) -> f64 {
        unsafe { (*self.inner).epsilon }
    }

    pub fn min_dt(&self) -> f64 {
        unsafe { (*self.inner).min_dt }
    }

    pub fn adaptive_mode(&self) -> Option<AdaptiveMode> {
        unsafe { AdaptiveMode::from_raw((*self.inner).adaptive_mode) }
    }
}

use rebound_bind as rb;

pub struct IntegratorLeapfrog<'a> {
    pub(crate) inner: *mut rb::reb_integrator_leapfrog,
    pub(crate) _marker: core::marker::PhantomData<&'a mut rb::reb_simulation>,
}

impl<'a> IntegratorLeapfrog<'a> {
    pub fn set_order(self, order: u32) -> Self {
        unsafe {
            (*self.inner).order = order;
        }
        self
    }

    pub fn order(&self) -> u32 {
        unsafe { (*self.inner).order }
    }
}

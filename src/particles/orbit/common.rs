#[derive(Default, Clone, Copy)]
pub enum SemiMajorAxisInput {
    #[default]
    Unset,
    SemiMajorAxis(f64),
    Period(f64),
    Conflicting,
}

impl SemiMajorAxisInput {
    pub fn set_semi_major_axis(self, semi_major_axis: f64) -> Self {
        match self {
            Self::Unset | Self::SemiMajorAxis(_) => Self::SemiMajorAxis(semi_major_axis),
            Self::Period(_) | Self::Conflicting => Self::Conflicting,
        }
    }

    pub fn set_period(self, period: f64) -> Self {
        match self {
            Self::Unset | Self::Period(_) => Self::Period(period),
            Self::SemiMajorAxis(_) | Self::Conflicting => Self::Conflicting,
        }
    }
}

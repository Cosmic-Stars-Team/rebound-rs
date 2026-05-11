use rebound_bind as rb;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SetError {
    #[error("Invalid value for `{field}`: {message}")]
    InvalidValue {
        field: &'static str,
        message: String,
    },
}

impl SetError {
    pub fn invalid(field: &'static str, message: impl Into<String>) -> Self {
        Self::InvalidValue {
            field,
            message: message.into(),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum IntegratorConfigError {
    #[error("Unknown integrator value.")]
    UnknownIntegrator,

    #[error("Field `{field}` is not supported by integrator `{integrator}`.")]
    UnsupportedField {
        integrator: &'static str,
        field: &'static str,
    },

    #[error("Failed to initialize integrator `{integrator}`.")]
    InitFailed { integrator: &'static str },
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum IntegrateError {
    #[error("Performing a single step, then switching to PAUSED.")]
    SingleStep,
    #[error("Screenshot is ready, send back, then finish integration.")]
    ScreenshotReady,
    #[error("Pause until visualization has taken a screenshot.")]
    Screenshot,
    #[error("Simulation is paused by visualization.")]
    Paused,
    #[error("Current timestep is the last one.")]
    LastStep,
    #[error("Simulation is currently running, no error occurred.")]
    Running,
    #[error("A generic error occurred and the integration was not successful.")]
    GenericError,
    #[error("The integration ends early because no particles are left in the simulation.")]
    NoParticles,
    #[error("The integration ends early because two particles had a close encounter.")]
    Encounter,
    #[error("The integration ends early because a particle escaped.")]
    Escape,
    #[error("User caused exit, simulation did not finish successfully.")]
    User,
    #[error("SIGINT received. Simulation stopped.")]
    Sigint,
    #[error("The integration ends early because two particles collided.")]
    Collision,

    #[error("Unknown REBOUND status code: {0}")]
    UnknownStatus(rb::REB_STATUS),
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum OrbitalElementsError {
    #[error("A primary particle is required. Set `primary` explicitly or provide a simulation.")]
    MissingPrimary,

    #[error("A gravitational constant is required. Set `g` explicitly or provide a simulation.")]
    MissingGravity,

    #[error(
        "A time value is required when using `time_of_periapsis_passage`. Set `time` explicitly or provide a simulation."
    )]
    MissingTime,

    #[error("One of `semi_major_axis` or `period` is required.")]
    MissingSemiMajorAxisOrPeriod,

    #[error("Only one of `semi_major_axis` or `period` may be provided.")]
    BothSemiMajorAxisAndPeriod,

    #[error("Only one of `argument_of_periapsis` or `periapsis_longitude` may be provided.")]
    BothArgumentOfPeriapsisAndPeriapsisLongitude,

    #[error(
        "Only one of `true_anomaly`, `mean_anomaly`, `eccentric_anomaly`, `mean_longitude`, `true_longitude`, or `time_of_periapsis_passage` may be provided."
    )]
    MultipleLongitudeInputs,

    #[error("Cannot set eccentricity exactly to 1.")]
    RadialOrbit,

    #[error("Eccentricity must be greater than or equal to zero.")]
    NegativeEccentricity,

    #[error("A bound orbit (`semi_major_axis > 0`) must have `eccentricity < 1`.")]
    BoundOrbitRequiresSubUnitEccentricity,

    #[error("An unbound orbit (`semi_major_axis < 0`) must have `eccentricity > 1`.")]
    UnboundOrbitRequiresSuperUnitEccentricity,

    #[error("The true anomaly is outside the valid range for this unbound orbit.")]
    UnboundTrueAnomalyOutOfRange,

    #[error("The primary particle must have a non-zero mass.")]
    PrimaryHasNoMass,

    #[error("Pal coordinates (`ix`, `iy`) are not valid. Squared sum exceeds 4.")]
    InvalidPalInclinationComponents,

    #[error("Particle data was unavailable.")]
    NullParticle,

    #[error("Unknown REBOUND orbital conversion error code: {0}")]
    UnknownOrbitError(i32),
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("Integration error: {0}")]
    Integrate(#[from] IntegrateError),

    #[error(transparent)]
    Set(#[from] SetError),

    #[error("Integrator config error: {0}")]
    IntegratorConfig(#[from] IntegratorConfigError),

    #[error(transparent)]
    OrbitalElements(#[from] OrbitalElementsError),

    #[error("Failed to allocate")]
    Allocation,

    #[error("Error: {0}")]
    Custom(String),
}

impl IntegrateError {
    pub fn from_reb_status(status: rb::REB_STATUS) -> Option<Self> {
        if status == rb::REB_STATUS_REB_STATUS_SUCCESS {
            None
        } else {
            Some(match status {
                rb::REB_STATUS_REB_STATUS_SINGLE_STEP => IntegrateError::SingleStep,
                rb::REB_STATUS_REB_STATUS_SCREENSHOT_READY => IntegrateError::ScreenshotReady,
                rb::REB_STATUS_REB_STATUS_SCREENSHOT => IntegrateError::Screenshot,
                rb::REB_STATUS_REB_STATUS_PAUSED => IntegrateError::Paused,
                rb::REB_STATUS_REB_STATUS_LAST_STEP => IntegrateError::LastStep,
                rb::REB_STATUS_REB_STATUS_RUNNING => IntegrateError::Running,
                rb::REB_STATUS_REB_STATUS_GENERIC_ERROR => IntegrateError::GenericError,
                rb::REB_STATUS_REB_STATUS_NO_PARTICLES => IntegrateError::NoParticles,
                rb::REB_STATUS_REB_STATUS_ENCOUNTER => IntegrateError::Encounter,
                rb::REB_STATUS_REB_STATUS_ESCAPE => IntegrateError::Escape,
                rb::REB_STATUS_REB_STATUS_USER => IntegrateError::User,
                rb::REB_STATUS_REB_STATUS_SIGINT => IntegrateError::Sigint,
                rb::REB_STATUS_REB_STATUS_COLLISION => IntegrateError::Collision,
                _ => IntegrateError::UnknownStatus(status),
            })
        }
    }
}

impl OrbitalElementsError {
    pub fn from_orbit_err(code: i32) -> Self {
        match code {
            1 => Self::RadialOrbit,
            2 => Self::NegativeEccentricity,
            3 => Self::BoundOrbitRequiresSubUnitEccentricity,
            4 => Self::UnboundOrbitRequiresSuperUnitEccentricity,
            5 => Self::UnboundTrueAnomalyOutOfRange,
            6 => Self::PrimaryHasNoMass,
            _ => Self::UnknownOrbitError(code),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

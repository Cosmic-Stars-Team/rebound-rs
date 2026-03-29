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
pub enum Error {
    #[error("Integration error: {0}")]
    Integrate(#[from] IntegrateError),

    #[error(transparent)]
    Set(#[from] SetError),

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

pub type Result<T> = std::result::Result<T, Error>;

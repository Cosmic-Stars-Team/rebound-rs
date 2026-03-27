use rebound_bind as rb;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
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

    #[error("Error: {0}")]
    Custom(String),
    #[error("Unknown REBOUND status code: {0}")]
    UnknownStatus(rb::REB_STATUS),
}

impl Error {
    pub fn from_reb_status(status: rb::REB_STATUS) -> Option<Self> {
        if status == rb::REB_STATUS_REB_STATUS_SUCCESS {
            None
        } else {
            Some(match status {
                rb::REB_STATUS_REB_STATUS_SINGLE_STEP => Error::SingleStep,
                rb::REB_STATUS_REB_STATUS_SCREENSHOT_READY => Error::ScreenshotReady,
                rb::REB_STATUS_REB_STATUS_SCREENSHOT => Error::Screenshot,
                rb::REB_STATUS_REB_STATUS_PAUSED => Error::Paused,
                rb::REB_STATUS_REB_STATUS_LAST_STEP => Error::LastStep,
                rb::REB_STATUS_REB_STATUS_RUNNING => Error::Running,
                rb::REB_STATUS_REB_STATUS_GENERIC_ERROR => Error::GenericError,
                rb::REB_STATUS_REB_STATUS_NO_PARTICLES => Error::NoParticles,
                rb::REB_STATUS_REB_STATUS_ENCOUNTER => Error::Encounter,
                rb::REB_STATUS_REB_STATUS_ESCAPE => Error::Escape,
                rb::REB_STATUS_REB_STATUS_USER => Error::User,
                rb::REB_STATUS_REB_STATUS_SIGINT => Error::Sigint,
                rb::REB_STATUS_REB_STATUS_COLLISION => Error::Collision,
                _ => Error::UnknownStatus(status),
            })
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

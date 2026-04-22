use std::f64::consts::PI;

use rebound_bind as rb;

use crate::{
    error::{OrbitalElementsError, Result},
    particles::{Particle, ParticleBuilder},
    simulation::{SimulationParticlesRead, SimulationSettingsRead, SimulationStateRead},
    utils,
};

use super::SemiMajorAxisInput;

#[derive(Default, Clone, Copy)]
enum ClassicalPeriapsisInput {
    #[default]
    Unset,
    ArgumentOfPeriapsis(f64),
    PeriapsisLongitude(f64),
    Conflicting,
}

impl ClassicalPeriapsisInput {
    fn set_argument_of_periapsis(self, argument_of_periapsis: f64) -> Self {
        match self {
            Self::Unset | Self::ArgumentOfPeriapsis(_) => {
                Self::ArgumentOfPeriapsis(argument_of_periapsis)
            }
            Self::PeriapsisLongitude(_) | Self::Conflicting => Self::Conflicting,
        }
    }

    fn set_periapsis_longitude(self, periapsis_longitude: f64) -> Self {
        match self {
            Self::Unset | Self::PeriapsisLongitude(_) => {
                Self::PeriapsisLongitude(periapsis_longitude)
            }
            Self::ArgumentOfPeriapsis(_) | Self::Conflicting => Self::Conflicting,
        }
    }
}

#[derive(Default, Clone, Copy)]
enum ClassicalLongitudeInput {
    #[default]
    Unset,
    TrueAnomaly(f64),
    MeanAnomaly(f64),
    EccentricAnomaly(f64),
    MeanLongitude(f64),
    TrueLongitude(f64),
    TimeOfPeriapsisPassage(f64),
    Conflicting,
}

impl ClassicalLongitudeInput {
    fn set_true_anomaly(self, true_anomaly: f64) -> Self {
        match self {
            Self::Unset | Self::TrueAnomaly(_) => Self::TrueAnomaly(true_anomaly),
            _ => Self::Conflicting,
        }
    }

    fn set_mean_anomaly(self, mean_anomaly: f64) -> Self {
        match self {
            Self::Unset | Self::MeanAnomaly(_) => Self::MeanAnomaly(mean_anomaly),
            _ => Self::Conflicting,
        }
    }

    fn set_eccentric_anomaly(self, eccentric_anomaly: f64) -> Self {
        match self {
            Self::Unset | Self::EccentricAnomaly(_) => Self::EccentricAnomaly(eccentric_anomaly),
            _ => Self::Conflicting,
        }
    }

    fn set_mean_longitude(self, mean_longitude: f64) -> Self {
        match self {
            Self::Unset | Self::MeanLongitude(_) => Self::MeanLongitude(mean_longitude),
            _ => Self::Conflicting,
        }
    }

    fn set_true_longitude(self, true_longitude: f64) -> Self {
        match self {
            Self::Unset | Self::TrueLongitude(_) => Self::TrueLongitude(true_longitude),
            _ => Self::Conflicting,
        }
    }

    fn set_time_of_periapsis_passage(self, time_of_periapsis_passage: f64) -> Self {
        match self {
            Self::Unset | Self::TimeOfPeriapsisPassage(_) => {
                Self::TimeOfPeriapsisPassage(time_of_periapsis_passage)
            }
            _ => Self::Conflicting,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct ClassicalOrbitalElementsBuilder {
    primary: Option<Particle>,
    g: Option<f64>,
    time: Option<f64>,
    hash: u32,
    mass: f64,
    radius: f64,
    eccentricity: f64,
    inclination: f64,
    ascending_node_longitude: f64,
    size: SemiMajorAxisInput,
    periapsis: ClassicalPeriapsisInput,
    longitude: ClassicalLongitudeInput,
}

impl ClassicalOrbitalElementsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_simulation_defaults<S>(mut self, simulation: &S) -> Self
    where
        S: SimulationParticlesRead + SimulationSettingsRead + SimulationStateRead + ?Sized,
    {
        if self.primary.is_none() {
            self.primary = Some(simulation.com());
        }
        if self.g.is_none() {
            self.g = Some(simulation.g());
        }
        if self.time.is_none() {
            self.time = Some(simulation.t());
        }
        self
    }

    pub fn set_primary(mut self, primary: Particle) -> Self {
        self.primary = Some(primary);
        self
    }

    pub fn set_g(mut self, g: f64) -> Self {
        self.g = Some(g);
        self
    }

    pub fn set_time(mut self, time: f64) -> Self {
        self.time = Some(time);
        self
    }

    pub fn set_hash(mut self, hash: u32) -> Self {
        self.hash = hash;
        self
    }

    pub fn set_hash_by_name(mut self, hash: &str) -> Self {
        self.hash = utils::hash(hash);
        self
    }

    pub fn set_mass(mut self, mass: f64) -> Self {
        self.mass = mass;
        self
    }

    pub fn set_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the semi-major axis.
    /// Upstream REBOUND/Python name: `a`.
    pub fn set_semi_major_axis(mut self, semi_major_axis: f64) -> Self {
        self.size = self.size.set_semi_major_axis(semi_major_axis);
        self
    }

    /// Sets the orbital period.
    /// Upstream REBOUND/Python name: `P`.
    pub fn set_period(mut self, period: f64) -> Self {
        self.size = self.size.set_period(period);
        self
    }

    /// Sets the eccentricity.
    /// Upstream REBOUND/Python name: `e`.
    pub fn set_eccentricity(mut self, eccentricity: f64) -> Self {
        self.eccentricity = eccentricity;
        self
    }

    /// Sets the inclination.
    /// Upstream REBOUND/Python name: `inc`.
    pub fn set_inclination(mut self, inclination: f64) -> Self {
        self.inclination = inclination;
        self
    }

    /// Sets the longitude of the ascending node.
    /// Upstream REBOUND/Python name: `Omega`.
    pub fn set_ascending_node_longitude(mut self, ascending_node_longitude: f64) -> Self {
        self.ascending_node_longitude = ascending_node_longitude;
        self
    }

    /// Sets the argument of periapsis.
    /// Upstream REBOUND/Python name: `omega`.
    pub fn set_argument_of_periapsis(mut self, argument_of_periapsis: f64) -> Self {
        self.periapsis = self
            .periapsis
            .set_argument_of_periapsis(argument_of_periapsis);
        self
    }

    /// Sets the longitude of periapsis.
    /// Upstream REBOUND/Python name: `pomega`.
    pub fn set_periapsis_longitude(mut self, periapsis_longitude: f64) -> Self {
        self.periapsis = self.periapsis.set_periapsis_longitude(periapsis_longitude);
        self
    }

    /// Sets the true anomaly.
    /// Upstream REBOUND/Python name: `f`.
    pub fn set_true_anomaly(mut self, true_anomaly: f64) -> Self {
        self.longitude = self.longitude.set_true_anomaly(true_anomaly);
        self
    }

    /// Sets the mean anomaly.
    /// Upstream REBOUND/Python name: `M`.
    pub fn set_mean_anomaly(mut self, mean_anomaly: f64) -> Self {
        self.longitude = self.longitude.set_mean_anomaly(mean_anomaly);
        self
    }

    /// Sets the eccentric anomaly.
    /// Upstream REBOUND/Python name: `E`.
    pub fn set_eccentric_anomaly(mut self, eccentric_anomaly: f64) -> Self {
        self.longitude = self.longitude.set_eccentric_anomaly(eccentric_anomaly);
        self
    }

    /// Sets the mean longitude.
    /// Upstream REBOUND/Python name: `l`.
    pub fn set_mean_longitude(mut self, mean_longitude: f64) -> Self {
        self.longitude = self.longitude.set_mean_longitude(mean_longitude);
        self
    }

    /// Sets the true longitude.
    /// Upstream REBOUND/Python name: `theta`.
    pub fn set_true_longitude(mut self, true_longitude: f64) -> Self {
        self.longitude = self.longitude.set_true_longitude(true_longitude);
        self
    }

    /// Sets the time of periapsis passage.
    /// Upstream REBOUND/Python name: `T`.
    pub fn set_time_of_periapsis_passage(mut self, time_of_periapsis_passage: f64) -> Self {
        self.longitude = self
            .longitude
            .set_time_of_periapsis_passage(time_of_periapsis_passage);
        self
    }

    fn resolve_primary(self) -> Result<Particle> {
        self.primary
            .ok_or(OrbitalElementsError::MissingPrimary.into())
    }

    fn resolve_g(self) -> Result<f64> {
        self.g.ok_or(OrbitalElementsError::MissingGravity.into())
    }

    fn resolve_time(self) -> Result<f64> {
        self.time.ok_or(OrbitalElementsError::MissingTime.into())
    }

    fn resolve_semi_major_axis(self, g: f64, primary: Particle) -> Result<f64> {
        match self.size {
            SemiMajorAxisInput::Unset => {
                Err(OrbitalElementsError::MissingSemiMajorAxisOrPeriod.into())
            }
            SemiMajorAxisInput::SemiMajorAxis(semi_major_axis) => Ok(semi_major_axis),
            SemiMajorAxisInput::Period(period) => {
                Ok((period * period * g * (primary.mass + self.mass) / (4.0 * PI * PI)).cbrt())
            }
            SemiMajorAxisInput::Conflicting => {
                Err(OrbitalElementsError::BothSemiMajorAxisAndPeriod.into())
            }
        }
    }

    fn resolve_argument_of_periapsis(
        self,
        inclination: f64,
        ascending_node_longitude: f64,
    ) -> Result<f64> {
        match self.periapsis {
            ClassicalPeriapsisInput::Unset => Ok(0.0),
            ClassicalPeriapsisInput::ArgumentOfPeriapsis(argument_of_periapsis) => {
                Ok(argument_of_periapsis)
            }
            ClassicalPeriapsisInput::PeriapsisLongitude(periapsis_longitude) => {
                if inclination.cos() > 0.0 {
                    Ok(periapsis_longitude - ascending_node_longitude)
                } else {
                    Ok(ascending_node_longitude - periapsis_longitude)
                }
            }
            ClassicalPeriapsisInput::Conflicting => {
                Err(OrbitalElementsError::BothArgumentOfPeriapsisAndPeriapsisLongitude.into())
            }
        }
    }

    fn resolve_true_anomaly(
        self,
        g: f64,
        primary: Particle,
        semi_major_axis: f64,
        argument_of_periapsis: f64,
    ) -> Result<f64> {
        let eccentricity = self.eccentricity;
        let inclination = self.inclination;
        let ascending_node_longitude = self.ascending_node_longitude;

        match self.longitude {
            ClassicalLongitudeInput::Unset => Ok(0.0),
            ClassicalLongitudeInput::TrueAnomaly(true_anomaly) => Ok(true_anomaly),
            ClassicalLongitudeInput::MeanAnomaly(mean_anomaly) => {
                Ok(unsafe { rb::reb_M_to_f(eccentricity, mean_anomaly) })
            }
            ClassicalLongitudeInput::EccentricAnomaly(eccentric_anomaly) => {
                Ok(unsafe { rb::reb_E_to_f(eccentricity, eccentric_anomaly) })
            }
            ClassicalLongitudeInput::MeanLongitude(mean_longitude) => {
                let mean_anomaly = if inclination.cos() > 0.0 {
                    mean_longitude - ascending_node_longitude - argument_of_periapsis
                } else {
                    ascending_node_longitude - argument_of_periapsis - mean_longitude
                };
                Ok(unsafe { rb::reb_M_to_f(eccentricity, mean_anomaly) })
            }
            ClassicalLongitudeInput::TrueLongitude(true_longitude) => {
                if inclination.cos() > 0.0 {
                    Ok(true_longitude - ascending_node_longitude - argument_of_periapsis)
                } else {
                    Ok(ascending_node_longitude - argument_of_periapsis - true_longitude)
                }
            }
            ClassicalLongitudeInput::TimeOfPeriapsisPassage(time_of_periapsis_passage) => {
                let time = self.resolve_time()?;
                let mean_motion = (g * (primary.mass + self.mass)
                    / (semi_major_axis * semi_major_axis * semi_major_axis).abs())
                .sqrt();
                let mean_anomaly = mean_motion * (time - time_of_periapsis_passage);
                Ok(unsafe { rb::reb_M_to_f(eccentricity, mean_anomaly) })
            }
            ClassicalLongitudeInput::Conflicting => {
                Err(OrbitalElementsError::MultipleLongitudeInputs.into())
            }
        }
    }
}

impl ParticleBuilder for ClassicalOrbitalElementsBuilder {
    fn with_simulation_defaults<S>(self, simulation: &S) -> Self
    where
        S: SimulationParticlesRead + SimulationSettingsRead + SimulationStateRead + ?Sized,
    {
        ClassicalOrbitalElementsBuilder::with_simulation_defaults(self, simulation)
    }

    fn build(self) -> Result<Particle> {
        let primary = self.resolve_primary()?;
        let g = self.resolve_g()?;
        let semi_major_axis = self.resolve_semi_major_axis(g, primary)?;
        let argument_of_periapsis =
            self.resolve_argument_of_periapsis(self.inclination, self.ascending_node_longitude)?;
        let true_anomaly =
            self.resolve_true_anomaly(g, primary, semi_major_axis, argument_of_periapsis)?;

        let mut err = 0;
        let particle = unsafe {
            rb::reb_particle_from_orbit_err(
                g,
                primary.into(),
                self.mass,
                semi_major_axis,
                self.eccentricity,
                self.inclination,
                self.ascending_node_longitude,
                argument_of_periapsis,
                true_anomaly,
                &mut err,
            )
        };

        if err != 0 {
            return Err(OrbitalElementsError::from_orbit_err(err).into());
        }

        let mut particle: Particle = particle.into();
        particle.hash = self.hash;
        particle.radius = self.radius;
        Ok(particle)
    }
}

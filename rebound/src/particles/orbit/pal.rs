use std::f64::consts::PI;

use rebound_bind as rb;

use crate::{
    error::{OrbitalElementsError, Result},
    particles::{Particle, ParticleBuilder},
    simulator::Simulation,
    utils,
};

use super::SemiMajorAxisInput;

#[derive(Clone, Copy, Default)]
pub struct PalOrbitalElementsBuilder {
    primary: Option<Particle>,
    g: Option<f64>,
    hash: u32,
    mass: f64,
    radius: f64,
    mean_longitude: f64,
    h: f64,
    k: f64,
    ix: f64,
    iy: f64,
    size: SemiMajorAxisInput,
}

impl PalOrbitalElementsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_simulation_defaults(mut self, simulation: &Simulation) -> Self {
        if self.primary.is_none() {
            self.primary = Some(simulation.com());
        }
        if self.g.is_none() {
            self.g = Some(simulation.g());
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

    /// Sets the mean longitude in Pal coordinates.
    /// Upstream REBOUND/Python name: `l`.
    pub fn set_mean_longitude(mut self, mean_longitude: f64) -> Self {
        self.mean_longitude = mean_longitude;
        self
    }

    /// Sets the Pal `h` coordinate.
    /// Upstream REBOUND/Python name: `h`.
    pub fn set_h(mut self, h: f64) -> Self {
        self.h = h;
        self
    }

    /// Sets the Pal `k` coordinate.
    /// Upstream REBOUND/Python name: `k`.
    pub fn set_k(mut self, k: f64) -> Self {
        self.k = k;
        self
    }

    /// Sets the Pal `ix` coordinate.
    /// Upstream REBOUND/Python name: `ix`.
    pub fn set_ix(mut self, ix: f64) -> Self {
        self.ix = ix;
        self
    }

    /// Sets the Pal `iy` coordinate.
    /// Upstream REBOUND/Python name: `iy`.
    pub fn set_iy(mut self, iy: f64) -> Self {
        self.iy = iy;
        self
    }

    fn resolve_primary(self) -> Result<Particle> {
        self.primary
            .ok_or(OrbitalElementsError::MissingPrimary.into())
    }

    fn resolve_g(self) -> Result<f64> {
        self.g.ok_or(OrbitalElementsError::MissingGravity.into())
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
}

impl ParticleBuilder for PalOrbitalElementsBuilder {
    fn with_simulation_defaults(self, simulation: &Simulation) -> Self {
        PalOrbitalElementsBuilder::with_simulation_defaults(self, simulation)
    }

    fn build(self) -> Result<Particle> {
        let primary = self.resolve_primary()?;
        let g = self.resolve_g()?;
        let semi_major_axis = self.resolve_semi_major_axis(g, primary)?;

        if (self.ix * self.ix + self.iy * self.iy) > 4.0 {
            return Err(OrbitalElementsError::InvalidPalInclinationComponents.into());
        }

        let particle = unsafe {
            rb::reb_particle_from_pal(
                g,
                primary.into(),
                self.mass,
                semi_major_axis,
                self.mean_longitude,
                self.k,
                self.h,
                self.ix,
                self.iy,
            )
        };

        let mut particle: Particle = particle.into();
        particle.hash = self.hash;
        particle.radius = self.radius;
        Ok(particle)
    }
}

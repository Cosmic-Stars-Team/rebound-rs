mod classical;
mod common;
mod pal;

pub use classical::ClassicalOrbitalElementsBuilder;
pub use pal::PalOrbitalElementsBuilder;

use common::SemiMajorAxisInput;
use rebound_bind::{self as rb, reb_orbit};

use crate::{
    Result,
    error::OrbitalElementsError,
    particles::{Particle, Vec3d},
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Orbit {
    pub d: f64,
    pub v: f64,
    pub h: f64,
    pub n: f64,
    pub rhill: f64,
    pub pal_h: f64,
    pub pal_k: f64,
    pub pal_ix: f64,
    pub pal_iy: f64,
    pub hvec: Vec3d,
    pub evec: Vec3d,
    pub period: f64,                    // P
    pub semi_major_axis: f64,           // a
    pub eccentricity: f64,              // e
    pub inclination: f64,               // inc
    pub ascending_node_longitude: f64,  // Omega
    pub argument_of_periapsis: f64,     // omega
    pub periapsis_longitude: f64,       // pomega
    pub true_anomaly: f64,              // f
    pub mean_anomaly: f64,              // M
    pub mean_longitude: f64,            // l
    pub true_longitude: f64,            // theta
    pub time_of_periapsis_passage: f64, // T
}

impl Orbit {
    pub fn try_from_particle(g: f64, particle: &Particle, primary: &Particle) -> Result<Self> {
        let raw_particle: rb::reb_particle = (*particle).into();
        let raw_primary: rb::reb_particle = (*primary).into();
        let mut err = 0;

        let orbit =
            unsafe { rb::reb_orbit_from_particle_err(g, raw_particle, raw_primary, &mut err) };

        if err != 0 {
            return Err(OrbitalElementsError::from_orbit_err(err).into());
        }

        Ok(orbit.into())
    }

    pub fn from_particle(
        g: f64,
        particle: &rb::reb_particle,
        primary: &rb::reb_particle,
    ) -> Option<Self> {
        let mut err = 0;
        let orbit = unsafe { rb::reb_orbit_from_particle_err(g, *particle, *primary, &mut err) };
        if err != 0 {
            return None;
        }
        Some(orbit.into())
    }
}

impl From<reb_orbit> for Orbit {
    fn from(value: reb_orbit) -> Self {
        Self {
            d: value.d,
            v: value.v,
            h: value.h,
            n: value.n,
            rhill: value.rhill,
            pal_h: value.pal_h,
            pal_k: value.pal_k,
            pal_ix: value.pal_ix,
            pal_iy: value.pal_iy,
            hvec: value.hvec.into(),
            evec: value.evec.into(),
            period: value.P,
            semi_major_axis: value.a,
            eccentricity: value.e,
            inclination: value.inc,
            ascending_node_longitude: value.Omega,
            argument_of_periapsis: value.omega,
            periapsis_longitude: value.pomega,
            true_anomaly: value.f,
            mean_anomaly: value.M,
            mean_longitude: value.l,
            true_longitude: value.theta,
            time_of_periapsis_passage: value.T,
        }
    }
}

impl From<Orbit> for reb_orbit {
    fn from(value: Orbit) -> Self {
        Self {
            d: value.d,
            v: value.v,
            h: value.h,
            n: value.n,
            rhill: value.rhill,
            pal_h: value.pal_h,
            pal_k: value.pal_k,
            pal_ix: value.pal_ix,
            pal_iy: value.pal_iy,
            hvec: value.hvec.into(),
            evec: value.evec.into(),
            P: value.period,
            a: value.semi_major_axis,
            e: value.eccentricity,
            inc: value.inclination,
            Omega: value.ascending_node_longitude,
            omega: value.argument_of_periapsis,
            pomega: value.periapsis_longitude,
            f: value.true_anomaly,
            M: value.mean_anomaly,
            l: value.mean_longitude,
            theta: value.true_longitude,
            T: value.time_of_periapsis_passage,
        }
    }
}

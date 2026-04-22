use rebound::{
    Result, create_particle,
    simulation::{
        Simulation, SimulationIntegratorWrite, SimulationParticlesRead, SimulationParticlesWrite,
        SimulationSettingsRead, SimulationStateRead,
    },
};

fn main() -> Result<()> {
    // This example is ported from the C version of REBOUND.
    // https://github.com/hannorein/rebound/blob/main/examples/simplest/problem.c

    let mut r = Simulation::new();
    r.add_particle(create_particle! {
        mass: 1.
    })?
    .add_particle(create_particle! {
        mass: 1e-3,
        a: 1.,
        e: 0.1,
    })?
    .add_particle(create_particle! {
        a: 1.4,
        e: 0.1,
    })?;

    r.integrate(100.)?;

    for particle in r.particles() {
        let pos = particle
            .position()
            .ok_or_else(|| rebound::Error::Custom("Particle position not found".into()))?;

        println!("{} {} {}", pos.0, pos.1, pos.2);
    }

    let primary = r
        .get_particle(0)
        .ok_or_else(|| rebound::Error::Custom("Primary particle not found".into()))?;

    for i in 1..r.n() {
        let orbit = r
            .get_particle(i)
            .ok_or_else(|| rebound::Error::Custom("Particle orbit not found".into()))?
            .into_orbit(r.g(), &primary)
            .ok_or_else(|| rebound::Error::Custom("Particle orbit not found".into()))?;

        println!(
            "{} {} {}",
            orbit.semi_major_axis, orbit.eccentricity, orbit.true_anomaly
        );
    }

    Ok(())
}

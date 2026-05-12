use rebound_rs::{
    Error, Result, create_particle,
    error::OrbitalElementsError,
    particles::Orbit,
    simulation::{
        Simulation, SimulationParticlesRead, SimulationParticlesWrite, SimulationSettingsRead,
    },
};

fn main() -> Result<()> {
    // This example is ported from the C version of REBOUND.
    // https://github.com/hannorein/rebound/blob/main/examples/orbital_elements/problem.c

    let mut r = Simulation::new();

    // Upstream C writes `struct reb_particle p; p.m = 1.;`.
    // In Rust, `create_particle!` starts from `Particle::default()`, so omitted
    // Cartesian fields are zero-initialized. This matches the safe C pattern
    // `struct reb_particle p = {0}; p.m = 1.;`.
    r.add_particle(create_particle! {
        mass: 1.,
    })?;

    let primary = r
        .get_particle(0)
        .ok_or_else(|| Error::Custom("Primary particle not found".into()))?
        .snapshot()
        .ok_or_else(|| Error::Custom("Primary particle data not found".into()))?;

    // Adding a particle from classical orbital elements requires these values.
    // The field names below use rebound-rs wrapper names; comments show the
    // corresponding upstream REBOUND C/Python names.
    let mass = 0.; // m
    let semi_major_axis = 0.1; // a
    let mut eccentricity = 0.2; // e
    let inclination = 0.3; // inc
    let ascending_node_longitude = 0.4; // Omega
    let argument_of_periapsis = 0.5; // omega
    let true_anomaly = 0.6; // f

    r.add_particle(create_particle! {
        classical,
        primary: primary,
        g: r.g(),
        mass: mass,
        semi_major_axis: semi_major_axis,
        eccentricity: eccentricity,
        inclination: inclination,
        ascending_node_longitude: ascending_node_longitude,
        argument_of_periapsis: argument_of_periapsis,
        true_anomaly: true_anomaly,
    })?;

    let particle = r
        .get_particle(1)
        .ok_or_else(|| Error::Custom("Particle 1 not found".into()))?;
    let orbit = Orbit::try_from_particles(r.g(), &particle, &primary)?;
    print_orbit(&orbit);

    // Upstream C also provides `_err` variants that write an integer error code
    // through an `int*`, such as error code 3 for a bound orbit with e > 1.
    // rebound-rs maps those integer codes to typed errors instead.
    eccentricity = 1.001;
    let particle = create_particle! {
        classical,
        primary: primary,
        g: r.g(),
        mass: mass,
        semi_major_axis: semi_major_axis,
        eccentricity: eccentricity,
        inclination: inclination,
        ascending_node_longitude: ascending_node_longitude,
        argument_of_periapsis: argument_of_periapsis,
        true_anomaly: true_anomaly,
    };

    match r.add_particle(particle) {
        Err(Error::OrbitalElements(
            OrbitalElementsError::BoundOrbitRequiresSubUnitEccentricity,
        )) => {
            eccentricity = 1. - 1.0e-15;
            r.add_particle(create_particle! {
                classical,
                primary: primary,
                g: r.g(),
                mass: mass,
                semi_major_axis: semi_major_axis,
                eccentricity: eccentricity,
                inclination: inclination,
                ascending_node_longitude: ascending_node_longitude,
                argument_of_periapsis: argument_of_periapsis,
                true_anomaly: true_anomaly,
            })?;
        }
        result => {
            result?;
        }
    }

    let particle = r
        .get_particle(2)
        .ok_or_else(|| Error::Custom("Particle 2 not found".into()))?;
    let orbit = Orbit::try_from_particles(r.g(), &particle, &primary)?;
    println!();
    print_orbit(&orbit);

    Ok(())
}

fn print_orbit(orbit: &Orbit) {
    println!("semi_major_axis = {:.16e}", orbit.semi_major_axis); // a
    println!("eccentricity = {:.16e}", orbit.eccentricity); // e
    println!("inclination = {:.16e}", orbit.inclination); // inc
    println!(
        "ascending_node_longitude = {:.16e}",
        orbit.ascending_node_longitude
    ); // Omega
    println!(
        "argument_of_periapsis = {:.16e}",
        orbit.argument_of_periapsis
    ); // omega
    println!("true_anomaly = {:.16e}", orbit.true_anomaly); // f
}

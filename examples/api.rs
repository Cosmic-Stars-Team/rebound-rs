use std::f64::consts::PI;

use rebound_rs::{Result, create_particle, particles::Orbit, utils};

fn main() -> Result<()> {
    let mut p = create_particle! {
        x: 1.,
        vy: 1.
    };

    println!(
        "Initial position: {:.6} {:.6} {:.6}\n",
        p.position.0, p.position.1, p.position.2
    );

    const G: f64 = 1.;
    const GM: f64 = G * 1.;
    let dt = PI;
    p.whfast_kepler_solver(GM, dt);

    println!(
        "Final position: {:.6} {:.6} {:.6}\n",
        p.position.0, p.position.1, p.position.2
    );

    let primary = create_particle! {
        mass: 1.,
    };
    let o = Orbit::try_from_particles(G, &p, &primary)?;

    println!("Semi-major axis: {:.6}", o.semi_major_axis);
    println!("Eccentricity: {:.6}", o.eccentricity);
    println!("Inclination: {:.6}", o.inclination);

    println!(
        "Eccentric anomaly: {:.6}",
        utils::orbit::mean_to_eccentric_anomaly(o.eccentricity, o.mean_anomaly)
    );

    Ok(())
}

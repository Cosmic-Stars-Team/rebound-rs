use std::f64::consts::PI;

use rebound::{
    Error, Result, create_particle,
    simulation::Simulation,
    types::{Rotation, Vec3d},
};

fn main() -> Result<()> {
    // This example is ported from the C version of REBOUND.
    // https://github.com/hannorein/rebound/blob/main/examples/rotations/problem.c

    // rebound-rs provides a Rotation struct (wrapper of `reb_rotation`).
    // Internally, it is implemented using quaternions but you don't need to understand how quaternions work!
    // Simply put: this struct can rotate a vector or an entire simulation.

    // The following example rotates the vector v around the x axis by 90 degrees:
    let axis = Vec3d(1., 0., 0.);
    let r1 = Rotation::init_angle_axis(PI / 2., axis);

    let v = Vec3d(1., 2., 3.);
    let v_rotated = v.rotate(r1);
    println!("v_rotated = {:?}", v_rotated);

    // You can rotate a particle (its position and velocity)
    let _p = create_particle! {
        mass: 1.,
        x: 1.,
        vy: 1.,
    }
    .irotate(r1);

    // You can also rotate all the particles in a simulation:
    let sim = Simulation::try_new()?
        .add_particle(create_particle! {
            mass: 1.
        })?
        .add_particle(create_particle! {
            mass: 1e-3,
            a: 1.,
            e: 0.1
        })?
        .irotate(r1);
    drop(sim);

    // You can chain rotations by multiplying them together.
    // Note that the order of rotations matters, just like the order matters when multiplying together two matrices.
    let axis_y = Vec3d(0., 1., 0.);
    let r2 = Rotation::init_angle_axis(PI / 2., axis_y);
    let r_combined = r1 * r2;
    let _v_rotated = v.rotate(r_combined);

    // You can easily calculate the inverse of rotations.
    let r_inverse = r_combined.inverse();
    let _v_rotated = v.rotate(r_inverse);

    // For celestial mechanics, we provide a special init method that uses the ascending node, inclination and longitude of periastron.
    // Applying this method to an orbit in the xy plane with the pericenter on the x axis gives the same result as initializing an particle with orbital parameters the "normal way".
    let ascending_node_longitude = 0.12; // Omega
    let inclination = 0.223; // inc
    let argument_of_periapsis = 0.345; // omega
    let r_orbit =
        Rotation::init_orbit(ascending_node_longitude, inclination, argument_of_periapsis);

    let sim = Simulation::try_new()?
        .add_particle(create_particle! {
            mass: 1.
        })?
        .add_particle(create_particle! {
            a: 1.,
            e: 0.001,
        })?
        .add_particle(create_particle! {
            a: 1.,
            e: 0.001,
            Omega: ascending_node_longitude,
            inc: inclination,
            omega: argument_of_periapsis,
        })?;

    // REBOUND also comes with a built-in constructor that generates a rotation which rotates a given vector to a new vector. For example:
    let mut p1 = sim
        .get_particle(1)
        .ok_or_else(|| Error::Custom("Cannot get particle 1".into()))?;
    let p2 = sim
        .get_particle(2)
        .ok_or_else(|| Error::Custom("Cannot get particle 2".into()))?;
    p1.irotate(r_orbit);
    println!("particle[1] = {:?}  {:?}", p1.position(), p1.velocity());
    println!("particle[2] = {:?}  {:?}", p2.position(), p2.velocity());
    drop(sim);

    let v1 = Vec3d(1., 0., 0.);
    let mut v2 = Vec3d(4., 5., 6.);

    let r3 = Rotation::init_from_to(v1, v2);
    let v_rotated = v1.rotate(r3);

    v2 = v2.normalize();

    println!("v2        = {:?}", v2);
    println!("v_rotated = {:?}", v_rotated);

    Ok(())
}

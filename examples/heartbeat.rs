use rebound_rs::{
    Result, create_particle,
    simulation::{
        Simulation, SimulationCallbacksWrite, SimulationIntegratorWrite, SimulationParticlesWrite,
        SimulationRefMut, SimulationSettingsWrite, SimulationStateRead,
    },
};

fn heartbeat(sim: SimulationRefMut<'_>) {
    println!("{:.6}", sim.t());
}

fn main() -> Result<()> {
    let mut sim = Simulation::try_new()?;

    sim.set_dt(0.1)?
        .set_heartbeat(heartbeat)
        .set_exact_finish_time(true);

    sim.add_particle(create_particle! {
        mass: 1.,
    })?
    .add_particle(create_particle! {
        a: 1.,
        e: 0.,
    })?
    .integrate(100.)?;

    Ok(())
}

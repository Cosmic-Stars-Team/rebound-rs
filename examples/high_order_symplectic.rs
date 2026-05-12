#![allow(clippy::excessive_precision)]

use rebound_rs::{
    Result, create_particle,
    simulation::{
        self, Integrator, Simulation, SimulationIntegratorWrite, SimulationParticlesWrite,
        SimulationSettingsWrite, SimulationToolsWrite, saba, whfast,
    },
};

const SS_POS: [[f64; 3]; 10] = [
    [
        3.256101656448802E-03,
        -1.951205394420489E-04,
        -1.478264728548705E-04,
    ],
    [
        -1.927589645545195E-01,
        2.588788361485397E-01,
        3.900432597062033E-02,
    ],
    [
        -5.976537074581466E-01,
        3.918678996109574E-01,
        3.990356741282203E-02,
    ],
    [
        -7.986189029000561E-01,
        -6.086873314992410E-01,
        -1.250824315650566E-04,
    ],
    [
        7.897942807177173E-01,
        1.266671734964037E+00,
        7.092292179885432E-03,
    ],
    [
        -4.314503046344270E+00,
        3.168094294126697E+00,
        8.331048545353310E-02,
    ],
    [
        -4.882304833383455E+00,
        -8.689263067189865E+00,
        3.453930436208210E-01,
    ],
    [
        1.917757033372740E+01,
        5.671738750949031E+00,
        -2.273858614425555E-01,
    ],
    [
        2.767031517959636E+01,
        -1.150331645280942E+01,
        -4.008018419157927E-01,
    ],
    [
        7.765250227278298E+00,
        -3.190996242617413E+01,
        1.168394015703735E+00,
    ],
];

const SS_VEL: [[f64; 3]; 10] = [
    [
        3.039963463108432E-06,
        6.030576499910942E-06,
        -7.992931269075703E-08,
    ],
    [
        -2.811550184725887E-02,
        -1.586532995282261E-02,
        1.282829413699522E-03,
    ],
    [
        -1.113090630745269E-02,
        -1.703310700277280E-02,
        4.089082927733997E-04,
    ],
    [
        1.012305635253317E-02,
        -1.376389620972473E-02,
        3.482505080431706E-07,
    ],
    [
        -1.135279609707971E-02,
        8.579013475676980E-03,
        4.582774369441005E-04,
    ],
    [
        -4.555986691913995E-03,
        -5.727124269621595E-03,
        1.257262404884127E-04,
    ],
    [
        4.559352462922572E-03,
        -2.748632232963112E-03,
        -1.337915989241807E-04,
    ],
    [
        -1.144087185031310E-03,
        3.588282323722787E-03,
        2.829006644043203E-05,
    ],
    [
        1.183702780101068E-03,
        2.917115980784960E-03,
        -8.714411604869349E-05,
    ],
    [
        3.112825364672655E-03,
        1.004673400082409E-04,
        -9.111652976208292E-04,
    ],
];

const SS_MASS: [f64; 10] = [
    1.988544e30,
    3.302e23,
    48.685e23,
    6.0477246e24,
    6.4185e23,
    1898.13e24,
    5.68319e26,
    86.8103e24,
    102.41e24,
    1.4639248e+22,
];

fn create_simulation() -> Result<Simulation> {
    let mut sim = Simulation::try_new()?;

    sim.set_g(1.0e-34).set_dt(4.)?;

    for i in 0..10 {
        let p = create_particle! {
            mass: SS_MASS[i],
            x: SS_POS[i][0],
            y: SS_POS[i][1],
            z: SS_POS[i][2],
            vx: SS_VEL[i][0],
            vy: SS_VEL[i][1],
            vz: SS_VEL[i][2],
        };

        sim.add_particle(p)?;
    }

    Ok(sim)
}

fn main() -> Result<()> {
    const TMAX: f64 = 1e5;

    {
        let mut r = create_simulation()?;
        r.set_integrator(Integrator::Whfast);

        simulation::set_integrator_config!(r, {
            safe_mode: 0,
            corrector: 17,
            kernel: whfast::Kernel::Lazy,
        })?;

        let e_init = r.energy();
        r.integrate(TMAX)?;
        let e_final = r.energy();

        println!(
            "Relative energy error WHCKL: {:.6e}",
            ((e_init - e_final) / e_init).abs()
        )
    }

    {
        let mut r = create_simulation()?;
        r.set_integrator(Integrator::Saba);

        simulation::set_integrator_config!(r, {
            type: saba::Type::Saba1064,
            safe_mode: 0,
        })?;

        let e_init = r.energy();
        r.integrate(TMAX)?;
        let e_final = r.energy();

        println!(
            "Relative energy error SABA(10,6,4):    {:.6e}",
            ((e_init - e_final) / e_init).abs()
        )
    }

    {
        let mut r = create_simulation()?;
        r.set_integrator(Integrator::Whfast);

        simulation::set_integrator_config!(r, {
            safe_mode: 0,
        })?;

        let e_init = r.energy();
        r.integrate(TMAX)?;
        let e_final = r.energy();

        println!(
            "Relative energy error WH:    {:.6e}",
            ((e_init - e_final) / e_init).abs()
        )
    }

    Ok(())
}

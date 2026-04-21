use crate::{
    Result,
    particles::{
        ClassicalOrbitalElementsBuilder, PalOrbitalElementsBuilder, Particle, ParticleBuilder,
    },
    types::Vec3d,
    utils,
};

impl Particle {
    pub fn set_hash(mut self, hash: u32) -> Self {
        self.hash = hash;
        self
    }

    pub fn set_hash_by_name(self, hash: &str) -> Self {
        self.set_hash(utils::hash(hash))
    }

    pub fn set_mass(mut self, mass: f64) -> Self {
        self.mass = mass;
        self
    }

    pub fn set_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn set_x(mut self, x: f64) -> Self {
        self.position.0 = x;
        self
    }

    pub fn set_y(mut self, y: f64) -> Self {
        self.position.1 = y;
        self
    }

    pub fn set_z(mut self, z: f64) -> Self {
        self.position.2 = z;
        self
    }

    pub fn set_position(mut self, x: f64, y: f64, z: f64) -> Self {
        self.position = Vec3d(x, y, z);
        self
    }

    pub fn set_vx(mut self, vx: f64) -> Self {
        self.velocity.0 = vx;
        self
    }

    pub fn set_vy(mut self, vy: f64) -> Self {
        self.velocity.1 = vy;
        self
    }

    pub fn set_vz(mut self, vz: f64) -> Self {
        self.velocity.2 = vz;
        self
    }

    pub fn set_velocity(mut self, vx: f64, vy: f64, vz: f64) -> Self {
        self.velocity = Vec3d(vx, vy, vz);
        self
    }

    pub fn with_simulation_defaults(self, _simulation: &crate::simulation::Simulation) -> Self {
        self
    }
}

impl ParticleBuilder for Particle {
    fn with_simulation_defaults(self, _simulation: &crate::simulation::Simulation) -> Self {
        self
    }

    fn build(self) -> Result<Particle> {
        Ok(self)
    }
}

#[allow(dead_code)]
#[doc(hidden)]
pub trait ParticleHashTarget: Sized {
    fn set_hash_value(self, hash: u32) -> Self;
    fn set_hash_name(self, hash: &str) -> Self;
}

impl ParticleHashTarget for Particle {
    fn set_hash_value(self, hash: u32) -> Self {
        self.set_hash(hash)
    }

    fn set_hash_name(self, hash: &str) -> Self {
        self.set_hash_by_name(hash)
    }
}

impl ParticleHashTarget for ClassicalOrbitalElementsBuilder {
    fn set_hash_value(self, hash: u32) -> Self {
        self.set_hash(hash)
    }

    fn set_hash_name(self, hash: &str) -> Self {
        self.set_hash_by_name(hash)
    }
}

impl ParticleHashTarget for PalOrbitalElementsBuilder {
    fn set_hash_value(self, hash: u32) -> Self {
        self.set_hash(hash)
    }

    fn set_hash_name(self, hash: &str) -> Self {
        self.set_hash_by_name(hash)
    }
}

#[allow(dead_code)]
#[doc(hidden)]
pub trait ParticleHashInput<T>
where
    T: ParticleHashTarget,
{
    fn apply_hash(self, target: T) -> T;
}

impl<T> ParticleHashInput<T> for u32
where
    T: ParticleHashTarget,
{
    fn apply_hash(self, target: T) -> T {
        target.set_hash_value(self)
    }
}

impl<T> ParticleHashInput<T> for &str
where
    T: ParticleHashTarget,
{
    fn apply_hash(self, target: T) -> T {
        target.set_hash_name(self)
    }
}

impl<T> ParticleHashInput<T> for String
where
    T: ParticleHashTarget,
{
    fn apply_hash(self, target: T) -> T {
        target.set_hash_name(&self)
    }
}

impl<T> ParticleHashInput<T> for &String
where
    T: ParticleHashTarget,
{
    fn apply_hash(self, target: T) -> T {
        target.set_hash_name(self)
    }
}

#[allow(dead_code)]
#[doc(hidden)]
pub fn _set_particle_hash<T, V>(target: T, value: V) -> T
where
    T: ParticleHashTarget,
    V: ParticleHashInput<T>,
{
    value.apply_hash(target)
}

#[macro_export]
macro_rules! create_particle {
    (classical, $($field:ident : $value:expr),* $(,)?) => {{
        let mut particle = $crate::particles::ClassicalOrbitalElementsBuilder::new();
        $(
            particle = $crate::create_particle!(@set_classical particle, $field, $value);
        )*
        particle
    }};

    (pal, $($field:ident : $value:expr),* $(,)?) => {{
        let mut particle = $crate::particles::PalOrbitalElementsBuilder::new();
        $(
            particle = $crate::create_particle!(@set_pal particle, $field, $value);
        )*
        particle
    }};

    ($($field:ident : $value:expr),* $(,)?) => {
        $crate::create_particle!(
            @detect
            []
            []
            []
            []
            []
            $($field : $value,)*
        )
    };

    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
    ) => {
        $crate::create_particle!(
            @dispatch
            [$($fields)*]
            [$($cartesian)*]
            [$($orbit)*]
            [$($pal)*]
            [$($classical)*]
        )
    };

    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        x : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* x : $value,]
            [cartesian $($cartesian)*]
            [$($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        y : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* y : $value,]
            [cartesian $($cartesian)*]
            [$($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        z : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* z : $value,]
            [cartesian $($cartesian)*]
            [$($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        vx : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* vx : $value,]
            [cartesian $($cartesian)*]
            [$($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        vy : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* vy : $value,]
            [cartesian $($cartesian)*]
            [$($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        vz : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* vz : $value,]
            [cartesian $($cartesian)*]
            [$($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };

    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        h : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* h : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [pal $($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        k : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* k : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [pal $($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        ix : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* ix : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [pal $($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        iy : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* iy : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [pal $($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };

    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        a : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* semi_major_axis : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        P : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* period : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        e : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* eccentricity : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        inc : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* inclination : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        Omega : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* ascending_node_longitude : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        omega : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* argument_of_periapsis : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        pomega : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* periapsis_longitude : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        f : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* true_anomaly : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        M : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* mean_anomaly : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        E : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* eccentric_anomaly : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        l : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* mean_longitude : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        theta : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* true_longitude : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        T : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* time_of_periapsis_passage : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        pal_h : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* h : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [pal $($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        pal_k : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* k : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [pal $($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        pal_ix : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* ix : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [pal $($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        pal_iy : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* iy : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [pal $($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        eccentricity : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* eccentricity : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        inclination : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* inclination : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        ascending_node_longitude : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* ascending_node_longitude : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        argument_of_periapsis : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* argument_of_periapsis : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        periapsis_longitude : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* periapsis_longitude : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        true_anomaly : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* true_anomaly : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        mean_anomaly : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* mean_anomaly : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        eccentric_anomaly : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* eccentric_anomaly : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        true_longitude : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* true_longitude : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        time_of_periapsis_passage : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* time_of_periapsis_passage : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        time : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* time : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [classical $($classical)*]
            $($rest)*
        )
    };

    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        simulation : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* simulation : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        sim : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* simulation : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        primary : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* primary : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        g : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* g : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        semi_major_axis : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* semi_major_axis : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        period : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* period : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        mean_longitude : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* mean_longitude : $value,]
            [$($cartesian)*]
            [orbit $($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };

    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        hash : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* hash : $value,]
            [$($cartesian)*]
            [$($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        mass : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* mass : $value,]
            [$($cartesian)*]
            [$($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };
    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        radius : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)* radius : $value,]
            [$($cartesian)*]
            [$($orbit)*]
            [$($pal)*]
            [$($classical)*]
            $($rest)*
        )
    };

    (@detect
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        $field:ident : $value:expr, $($rest:tt)*
    ) => {
        compile_error!(concat!(
            "Unsupported field for create_particle!: ",
            stringify!($field),
            ". supported fields: simulation, sim, hash, mass, radius, x, y, z, vx, vy, vz, primary, g, semi_major_axis, period, time, eccentricity, inclination, ascending_node_longitude, argument_of_periapsis, periapsis_longitude, true_anomaly, mean_anomaly, eccentric_anomaly, mean_longitude, true_longitude, time_of_periapsis_passage, h, k, ix, iy"
        ));
    };

    (@dispatch
        [$( $field:ident : $value:expr, )*]
        [$cartesian:tt $($cartesian_rest:tt)*]
        [$orbit:tt $($orbit_rest:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
    ) => {
        compile_error!(
            "create_particle! cannot mix cartesian coordinates with orbital element fields"
        );
    };
    (@dispatch
        [$( $field:ident : $value:expr, )*]
        [$($cartesian:tt)*]
        [$orbit:tt $($orbit_rest:tt)*]
        [$pal:tt $($pal_rest:tt)*]
        [$classical:tt $($classical_rest:tt)*]
    ) => {
        compile_error!(
            "create_particle! cannot mix Pal coordinates with classical orbital element fields"
        );
    };
    (@dispatch
        [$( $field:ident : $value:expr, )*]
        [$($cartesian:tt)*]
        [$orbit:tt $($orbit_rest:tt)*]
        [$pal:tt $($pal_rest:tt)*]
        []
    ) => {{
        let mut particle = $crate::particles::PalOrbitalElementsBuilder::new();
        $(
            particle = $crate::create_particle!(@set_pal particle, $field, $value);
        )*
        particle
    }};
    (@dispatch
        [$( $field:ident : $value:expr, )*]
        [$($cartesian:tt)*]
        [$orbit:tt $($orbit_rest:tt)*]
        []
        [$($classical:tt)*]
    ) => {{
        let mut particle = $crate::particles::ClassicalOrbitalElementsBuilder::new();
        $(
            particle = $crate::create_particle!(@set_classical particle, $field, $value);
        )*
        particle
    }};
    (@dispatch
        [$( $field:ident : $value:expr, )*]
        [$($cartesian:tt)*]
        []
        []
        []
    ) => {{
        let mut particle = $crate::particles::Particle::new();
        $(
            particle = $crate::create_particle!(@set particle, $field, $value);
        )*
        particle
    }};

    (@set_common $particle:ident, hash, $value:expr) => {
        $crate::particles::_set_particle_hash($particle, $value)
    };
    (@set_common $particle:ident, mass, $value:expr) => {
        $particle.set_mass($value)
    };
    (@set_common $particle:ident, radius, $value:expr) => {
        $particle.set_radius($value)
    };
    (@set_common $particle:ident, semi_major_axis, $value:expr) => {
        $particle.set_semi_major_axis($value)
    };
    (@set_common $particle:ident, period, $value:expr) => {
        $particle.set_period($value)
    };
    (@set_common $particle:ident, simulation, $value:expr) => {
        $particle.with_simulation_defaults($value)
    };
    (@set_common $particle:ident, sim, $value:expr) => {
        $particle.with_simulation_defaults($value)
    };
    (@set_common $particle:ident, primary, $value:expr) => {
        $particle.set_primary($value)
    };
    (@set_common $particle:ident, g, $value:expr) => {
        $particle.set_g($value)
    };

    (@set $particle:ident, hash, $value:expr) => {
        $crate::create_particle!(@set_common $particle, hash, $value)
    };
    (@set $particle:ident, mass, $value:expr) => {
        $crate::create_particle!(@set_common $particle, mass, $value)
    };
    (@set $particle:ident, radius, $value:expr) => {
        $crate::create_particle!(@set_common $particle, radius, $value)
    };
    (@set $particle:ident, simulation, $value:expr) => {
        $crate::create_particle!(@set_common $particle, simulation, $value)
    };
    (@set $particle:ident, sim, $value:expr) => {
        $crate::create_particle!(@set_common $particle, sim, $value)
    };
    (@set $particle:ident, x, $value:expr) => {
        $particle.set_x($value)
    };
    (@set $particle:ident, y, $value:expr) => {
        $particle.set_y($value)
    };
    (@set $particle:ident, z, $value:expr) => {
        $particle.set_z($value)
    };
    (@set $particle:ident, vx, $value:expr) => {
        $particle.set_vx($value)
    };
    (@set $particle:ident, vy, $value:expr) => {
        $particle.set_vy($value)
    };
    (@set $particle:ident, vz, $value:expr) => {
        $particle.set_vz($value)
    };
    (@set $particle:ident, $field:ident, $value:expr) => {
        compile_error!(concat!(
            "Unsupported field for create_particle!: ",
            stringify!($field),
            ". supported fields: simulation, sim, hash, mass, radius, x, y, z, vx, vy, vz"
        ));
    };

    (@set_classical $particle:ident, hash, $value:expr) => {
        $crate::create_particle!(@set_common $particle, hash, $value)
    };
    (@set_classical $particle:ident, mass, $value:expr) => {
        $crate::create_particle!(@set_common $particle, mass, $value)
    };
    (@set_classical $particle:ident, radius, $value:expr) => {
        $crate::create_particle!(@set_common $particle, radius, $value)
    };
    (@set_classical $particle:ident, a, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, semi_major_axis, $value)
    };
    (@set_classical $particle:ident, P, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, period, $value)
    };
    (@set_classical $particle:ident, semi_major_axis, $value:expr) => {
        $crate::create_particle!(@set_common $particle, semi_major_axis, $value)
    };
    (@set_classical $particle:ident, period, $value:expr) => {
        $crate::create_particle!(@set_common $particle, period, $value)
    };
    (@set_classical $particle:ident, simulation, $value:expr) => {
        $crate::create_particle!(@set_common $particle, simulation, $value)
    };
    (@set_classical $particle:ident, sim, $value:expr) => {
        $crate::create_particle!(@set_common $particle, sim, $value)
    };
    (@set_classical $particle:ident, primary, $value:expr) => {
        $crate::create_particle!(@set_common $particle, primary, $value)
    };
    (@set_classical $particle:ident, g, $value:expr) => {
        $crate::create_particle!(@set_common $particle, g, $value)
    };
    (@set_classical $particle:ident, time, $value:expr) => {
        $particle.set_time($value)
    };
    (@set_classical $particle:ident, e, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, eccentricity, $value)
    };
    (@set_classical $particle:ident, eccentricity, $value:expr) => {
        $particle.set_eccentricity($value)
    };
    (@set_classical $particle:ident, inc, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, inclination, $value)
    };
    (@set_classical $particle:ident, inclination, $value:expr) => {
        $particle.set_inclination($value)
    };
    (@set_classical $particle:ident, Omega, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, ascending_node_longitude, $value)
    };
    (@set_classical $particle:ident, ascending_node_longitude, $value:expr) => {
        $particle.set_ascending_node_longitude($value)
    };
    (@set_classical $particle:ident, omega, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, argument_of_periapsis, $value)
    };
    (@set_classical $particle:ident, argument_of_periapsis, $value:expr) => {
        $particle.set_argument_of_periapsis($value)
    };
    (@set_classical $particle:ident, pomega, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, periapsis_longitude, $value)
    };
    (@set_classical $particle:ident, periapsis_longitude, $value:expr) => {
        $particle.set_periapsis_longitude($value)
    };
    (@set_classical $particle:ident, f, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, true_anomaly, $value)
    };
    (@set_classical $particle:ident, true_anomaly, $value:expr) => {
        $particle.set_true_anomaly($value)
    };
    (@set_classical $particle:ident, M, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, mean_anomaly, $value)
    };
    (@set_classical $particle:ident, mean_anomaly, $value:expr) => {
        $particle.set_mean_anomaly($value)
    };
    (@set_classical $particle:ident, E, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, eccentric_anomaly, $value)
    };
    (@set_classical $particle:ident, eccentric_anomaly, $value:expr) => {
        $particle.set_eccentric_anomaly($value)
    };
    (@set_classical $particle:ident, l, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, mean_longitude, $value)
    };
    (@set_classical $particle:ident, mean_longitude, $value:expr) => {
        $particle.set_mean_longitude($value)
    };
    (@set_classical $particle:ident, theta, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, true_longitude, $value)
    };
    (@set_classical $particle:ident, true_longitude, $value:expr) => {
        $particle.set_true_longitude($value)
    };
    (@set_classical $particle:ident, T, $value:expr) => {
        $crate::create_particle!(@set_classical $particle, time_of_periapsis_passage, $value)
    };
    (@set_classical $particle:ident, time_of_periapsis_passage, $value:expr) => {
        $particle.set_time_of_periapsis_passage($value)
    };
    (@set_classical $particle:ident, $field:ident, $value:expr) => {
        compile_error!(concat!(
            "Unsupported field for create_particle! classical mode: ",
            stringify!($field),
            ". supported fields: simulation, sim, primary, g, time, hash, mass, radius, semi_major_axis, period, eccentricity, inclination, ascending_node_longitude, argument_of_periapsis, periapsis_longitude, true_anomaly, mean_anomaly, eccentric_anomaly, mean_longitude, true_longitude, time_of_periapsis_passage"
        ));
    };

    (@set_pal $particle:ident, hash, $value:expr) => {
        $crate::create_particle!(@set_common $particle, hash, $value)
    };
    (@set_pal $particle:ident, mass, $value:expr) => {
        $crate::create_particle!(@set_common $particle, mass, $value)
    };
    (@set_pal $particle:ident, radius, $value:expr) => {
        $crate::create_particle!(@set_common $particle, radius, $value)
    };
    (@set_pal $particle:ident, a, $value:expr) => {
        $crate::create_particle!(@set_pal $particle, semi_major_axis, $value)
    };
    (@set_pal $particle:ident, P, $value:expr) => {
        $crate::create_particle!(@set_pal $particle, period, $value)
    };
    (@set_pal $particle:ident, semi_major_axis, $value:expr) => {
        $crate::create_particle!(@set_common $particle, semi_major_axis, $value)
    };
    (@set_pal $particle:ident, period, $value:expr) => {
        $crate::create_particle!(@set_common $particle, period, $value)
    };
    (@set_pal $particle:ident, simulation, $value:expr) => {
        $crate::create_particle!(@set_common $particle, simulation, $value)
    };
    (@set_pal $particle:ident, sim, $value:expr) => {
        $crate::create_particle!(@set_common $particle, sim, $value)
    };
    (@set_pal $particle:ident, primary, $value:expr) => {
        $crate::create_particle!(@set_common $particle, primary, $value)
    };
    (@set_pal $particle:ident, g, $value:expr) => {
        $crate::create_particle!(@set_common $particle, g, $value)
    };
    (@set_pal $particle:ident, l, $value:expr) => {
        $crate::create_particle!(@set_pal $particle, mean_longitude, $value)
    };
    (@set_pal $particle:ident, mean_longitude, $value:expr) => {
        $particle.set_mean_longitude($value)
    };
    (@set_pal $particle:ident, pal_h, $value:expr) => {
        $crate::create_particle!(@set_pal $particle, h, $value)
    };
    (@set_pal $particle:ident, h, $value:expr) => {
        $particle.set_h($value)
    };
    (@set_pal $particle:ident, pal_k, $value:expr) => {
        $crate::create_particle!(@set_pal $particle, k, $value)
    };
    (@set_pal $particle:ident, k, $value:expr) => {
        $particle.set_k($value)
    };
    (@set_pal $particle:ident, pal_ix, $value:expr) => {
        $crate::create_particle!(@set_pal $particle, ix, $value)
    };
    (@set_pal $particle:ident, ix, $value:expr) => {
        $particle.set_ix($value)
    };
    (@set_pal $particle:ident, pal_iy, $value:expr) => {
        $crate::create_particle!(@set_pal $particle, iy, $value)
    };
    (@set_pal $particle:ident, iy, $value:expr) => {
        $particle.set_iy($value)
    };
    (@set_pal $particle:ident, $field:ident, $value:expr) => {
        compile_error!(concat!(
            "Unsupported field for create_particle! pal mode: ",
            stringify!($field),
            ". supported fields: simulation, sim, primary, g, hash, mass, radius, semi_major_axis, period, mean_longitude, h, k, ix, iy"
        ));
    };
}

#[cfg(test)]
mod tests {
    use crate::types::Vec3d;

    #[test]
    fn macro_test() {
        let p1 = create_particle! {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            vx: 0.1,
            vy: 0.2,
            vz: 0.3,
        };
        assert_eq!(p1.mass, 0.0);
        assert_eq!(p1.position, Vec3d(1.0, 2.0, 3.0));
        assert_eq!(p1.velocity, Vec3d(0.1, 0.2, 0.3));

        let p2 = create_particle! {
            mass: 1.0,
        };
        assert_eq!(p2.mass, 1.0);
        assert_eq!(p2.position, Vec3d(0.0, 0.0, 0.0));
        assert_eq!(p2.velocity, Vec3d(0.0, 0.0, 0.0));
    }
}

use crate::{
    Result,
    particles::{
        ClassicalOrbitalElementsBuilder, IntoParticle, Orbit, PalOrbitalElementsBuilder, Particle,
        ParticleRead,
    },
    simulation::{SimulationParticlesRead, SimulationSettingsRead, SimulationStateRead},
    types::{Rotation, Vec3d},
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

    pub fn set_position_vec3d(mut self, position: Vec3d) -> Self {
        self.position = position;
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

    pub fn set_velocity_vec3d(mut self, velocity: Vec3d) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn set_ax(mut self, ax: f64) -> Self {
        self.acceleration.0 = ax;
        self
    }

    pub fn set_ay(mut self, ay: f64) -> Self {
        self.acceleration.1 = ay;
        self
    }

    pub fn set_az(mut self, az: f64) -> Self {
        self.acceleration.2 = az;
        self
    }

    pub fn set_acceleration(mut self, ax: f64, ay: f64, az: f64) -> Self {
        self.acceleration = Vec3d(ax, ay, az);
        self
    }

    pub fn set_acceleration_vec3d(mut self, acceleration: Vec3d) -> Self {
        self.acceleration = acceleration;
        self
    }

    pub fn irotate(self, rotation: Rotation) -> Self {
        let mut pos = self.position;
        pos.irotate(rotation);
        let particle = self.set_position_vec3d(pos);

        let mut vel = particle.velocity;
        vel.irotate(rotation);
        particle.set_velocity_vec3d(vel)
    }

    pub fn orbit(&self, g: f64, primary: &impl ParticleRead) -> Option<Orbit> {
        Orbit::from_particles(g, self, primary)
    }
}

impl IntoParticle for Particle {
    fn with_simulation_defaults<S>(self, _simulation: &S) -> Self
    where
        S: SimulationParticlesRead + SimulationSettingsRead + SimulationStateRead + ?Sized,
    {
        self
    }

    fn into_particle(self) -> Result<Particle> {
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

    (@mark_cartesian
        [$($fields:tt)*]
        [$($cartesian:tt)*]
        [$($orbit:tt)*]
        [$($pal:tt)*]
        [$($classical:tt)*]
        $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @detect
            [$($fields)*]
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
            @mark_cartesian
            [$($fields)* x : $value,]
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
        y : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @mark_cartesian
            [$($fields)* y : $value,]
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
        z : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @mark_cartesian
            [$($fields)* z : $value,]
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
        position : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @mark_cartesian
            [$($fields)* position : $value,]
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
        vx : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @mark_cartesian
            [$($fields)* vx : $value,]
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
        vy : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @mark_cartesian
            [$($fields)* vy : $value,]
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
        vz : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @mark_cartesian
            [$($fields)* vz : $value,]
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
        velocity : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @mark_cartesian
            [$($fields)* velocity : $value,]
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
        ax : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @mark_cartesian
            [$($fields)* ax : $value,]
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
        ay : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @mark_cartesian
            [$($fields)* ay : $value,]
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
        az : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @mark_cartesian
            [$($fields)* az : $value,]
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
        acceleration : $value:expr, $($rest:tt)*
    ) => {
        $crate::create_particle!(
            @mark_cartesian
            [$($fields)* acceleration : $value,]
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
            ". supported fields: simulation, sim, hash, mass, radius, x, y, z, position, vx, vy, vz, velocity, ax, ay, az, acceleration, primary, g, semi_major_axis, period, time, eccentricity, inclination, ascending_node_longitude, argument_of_periapsis, periapsis_longitude, true_anomaly, mean_anomaly, eccentric_anomaly, mean_longitude, true_longitude, time_of_periapsis_passage, h, k, ix, iy"
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
        $crate::particles::IntoParticle::with_simulation_defaults($particle, $value)
    };
    (@set_common $particle:ident, sim, $value:expr) => {
        $crate::particles::IntoParticle::with_simulation_defaults($particle, $value)
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
    (@set $particle:ident, position, $value:expr) => {
        $particle.set_position_vec3d($value)
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
    (@set $particle:ident, velocity, $value:expr) => {
        $particle.set_velocity_vec3d($value)
    };
    (@set $particle:ident, ax, $value:expr) => {
        $particle.set_ax($value)
    };
    (@set $particle:ident, ay, $value:expr) => {
        $particle.set_ay($value)
    };
    (@set $particle:ident, az, $value:expr) => {
        $particle.set_az($value)
    };
    (@set $particle:ident, acceleration, $value:expr) => {
        $particle.set_acceleration_vec3d($value)
    };
    (@set $particle:ident, $field:ident, $value:expr) => {
        compile_error!(concat!(
            "Unsupported field for create_particle!: ",
            stringify!($field),
            ". supported fields: simulation, sim, hash, mass, radius, x, y, z, position, vx, vy, vz, velocity, ax, ay, az, acceleration"
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
    use crate::{particles::IntoParticle, types::Vec3d};

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
        assert_eq!(p1.acceleration, Vec3d::default());

        let p2 = create_particle! {
            mass: 1.0,
        };
        assert_eq!(p2.mass, 1.0);
        assert_eq!(p2.position, Vec3d(0.0, 0.0, 0.0));
        assert_eq!(p2.velocity, Vec3d(0.0, 0.0, 0.0));
        assert_eq!(p2.acceleration, Vec3d::default());
    }

    #[test]
    fn particle_vec3d_setters_update_vector_fields() {
        let particle = crate::particles::Particle::new()
            .set_position_vec3d(Vec3d(1.0, 2.0, 3.0))
            .set_velocity_vec3d(Vec3d(4.0, 5.0, 6.0))
            .set_acceleration_vec3d(Vec3d(7.0, 8.0, 9.0));

        assert_eq!(particle.position, Vec3d(1.0, 2.0, 3.0));
        assert_eq!(particle.velocity, Vec3d(4.0, 5.0, 6.0));
        assert_eq!(particle.acceleration, Vec3d(7.0, 8.0, 9.0));
    }

    #[test]
    fn particle_orbit_accepts_owned_primary() {
        let primary = create_particle! {
            mass: 1.0,
        };
        let particle = create_particle! {
            classical,
            primary: primary,
            g: 1.0,
            semi_major_axis: 1.0,
        }
        .into_particle()
        .unwrap();

        let orbit = particle.orbit(1.0, &primary).unwrap();

        assert!((orbit.semi_major_axis - 1.0).abs() < 1e-12);
    }

    #[test]
    fn macro_accepts_vec3d_cartesian_fields() {
        let particle = create_particle! {
            position: Vec3d(1.0, 2.0, 3.0),
            velocity: Vec3d(4.0, 5.0, 6.0),
            acceleration: Vec3d(7.0, 8.0, 9.0),
        };

        assert_eq!(particle.position, Vec3d(1.0, 2.0, 3.0));
        assert_eq!(particle.velocity, Vec3d(4.0, 5.0, 6.0));
        assert_eq!(particle.acceleration, Vec3d(7.0, 8.0, 9.0));
    }

    #[test]
    fn macro_accepts_acceleration_components() {
        let particle = create_particle! {
            ax: 1.0,
            ay: 2.0,
            az: 3.0,
        };

        assert_eq!(particle.acceleration, Vec3d(1.0, 2.0, 3.0));
    }

    #[test]
    fn orbital_builders_return_particles_with_default_acceleration() {
        let primary = create_particle! {
            mass: 1.0,
        };

        let classical = create_particle! {
            primary: primary,
            g: 1.0,
            time: 0.0,
            semi_major_axis: 1.0,
        }
        .into_particle()
        .unwrap();
        assert_eq!(classical.acceleration, Vec3d::default());

        let pal = create_particle! {
            pal,
            primary: primary,
            g: 1.0,
            semi_major_axis: 1.0,
        }
        .into_particle()
        .unwrap();
        assert_eq!(pal.acceleration, Vec3d::default());
    }
}

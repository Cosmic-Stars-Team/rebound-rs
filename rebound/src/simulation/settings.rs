use crate::error::{Result, SetError};

use super::{SimulationRead, SimulationWrite};
use rebound_bind as rb;

// TODO: Confirm that the verification boundaries are accurate.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Collision {
    None = rb::reb_simulation_REB_COLLISION_NONE as isize,
    Direct = rb::reb_simulation_REB_COLLISION_DIRECT as isize,
    Tree = rb::reb_simulation_REB_COLLISION_TREE as isize,
    Line = rb::reb_simulation_REB_COLLISION_LINE as isize,
    LineTree = rb::reb_simulation_REB_COLLISION_LINETREE as isize,
}

impl From<Collision> for rb::reb_simulation__bindgen_ty_1 {
    fn from(value: Collision) -> Self {
        match value {
            Collision::None => rb::reb_simulation_REB_COLLISION_NONE,
            Collision::Direct => rb::reb_simulation_REB_COLLISION_DIRECT,
            Collision::Tree => rb::reb_simulation_REB_COLLISION_TREE,
            Collision::Line => rb::reb_simulation_REB_COLLISION_LINE,
            Collision::LineTree => rb::reb_simulation_REB_COLLISION_LINETREE,
        }
    }
}

impl Collision {
    fn from_raw(value: rb::reb_simulation__bindgen_ty_1) -> Option<Self> {
        match value {
            rb::reb_simulation_REB_COLLISION_NONE => Some(Self::None),
            rb::reb_simulation_REB_COLLISION_DIRECT => Some(Self::Direct),
            rb::reb_simulation_REB_COLLISION_TREE => Some(Self::Tree),
            rb::reb_simulation_REB_COLLISION_LINE => Some(Self::Line),
            rb::reb_simulation_REB_COLLISION_LINETREE => Some(Self::LineTree),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Boundary {
    None = rb::reb_simulation_REB_BOUNDARY_NONE as isize,
    Open = rb::reb_simulation_REB_BOUNDARY_OPEN as isize,
    Periodic = rb::reb_simulation_REB_BOUNDARY_PERIODIC as isize,
    Shear = rb::reb_simulation_REB_BOUNDARY_SHEAR as isize,
}

impl From<Boundary> for rb::reb_simulation__bindgen_ty_3 {
    fn from(value: Boundary) -> Self {
        match value {
            Boundary::None => rb::reb_simulation_REB_BOUNDARY_NONE,
            Boundary::Open => rb::reb_simulation_REB_BOUNDARY_OPEN,
            Boundary::Periodic => rb::reb_simulation_REB_BOUNDARY_PERIODIC,
            Boundary::Shear => rb::reb_simulation_REB_BOUNDARY_SHEAR,
        }
    }
}

impl Boundary {
    fn from_raw(value: rb::reb_simulation__bindgen_ty_3) -> Option<Self> {
        match value {
            rb::reb_simulation_REB_BOUNDARY_NONE => Some(Self::None),
            rb::reb_simulation_REB_BOUNDARY_OPEN => Some(Self::Open),
            rb::reb_simulation_REB_BOUNDARY_PERIODIC => Some(Self::Periodic),
            rb::reb_simulation_REB_BOUNDARY_SHEAR => Some(Self::Shear),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gravity {
    None = rb::reb_simulation_REB_GRAVITY_NONE as isize,
    Basic = rb::reb_simulation_REB_GRAVITY_BASIC as isize,
    Compensated = rb::reb_simulation_REB_GRAVITY_COMPENSATED as isize,
    Tree = rb::reb_simulation_REB_GRAVITY_TREE as isize,
    Mercurius = rb::reb_simulation_REB_GRAVITY_MERCURIUS as isize,
    Jacobi = rb::reb_simulation_REB_GRAVITY_JACOBI as isize,
    Trace = rb::reb_simulation_REB_GRAVITY_TRACE as isize,
}

impl From<Gravity> for rb::reb_simulation__bindgen_ty_4 {
    fn from(value: Gravity) -> Self {
        match value {
            Gravity::None => rb::reb_simulation_REB_GRAVITY_NONE,
            Gravity::Basic => rb::reb_simulation_REB_GRAVITY_BASIC,
            Gravity::Compensated => rb::reb_simulation_REB_GRAVITY_COMPENSATED,
            Gravity::Tree => rb::reb_simulation_REB_GRAVITY_TREE,
            Gravity::Mercurius => rb::reb_simulation_REB_GRAVITY_MERCURIUS,
            Gravity::Jacobi => rb::reb_simulation_REB_GRAVITY_JACOBI,
            Gravity::Trace => rb::reb_simulation_REB_GRAVITY_TRACE,
        }
    }
}

impl Gravity {
    fn from_raw(value: rb::reb_simulation__bindgen_ty_4) -> Option<Self> {
        match value {
            rb::reb_simulation_REB_GRAVITY_NONE => Some(Self::None),
            rb::reb_simulation_REB_GRAVITY_BASIC => Some(Self::Basic),
            rb::reb_simulation_REB_GRAVITY_COMPENSATED => Some(Self::Compensated),
            rb::reb_simulation_REB_GRAVITY_TREE => Some(Self::Tree),
            rb::reb_simulation_REB_GRAVITY_MERCURIUS => Some(Self::Mercurius),
            rb::reb_simulation_REB_GRAVITY_JACOBI => Some(Self::Jacobi),
            rb::reb_simulation_REB_GRAVITY_TRACE => Some(Self::Trace),
            _ => None,
        }
    }
}

pub trait SimulationSettingsRead: SimulationRead {
    fn g(&self) -> f64 {
        unsafe { (*self.raw()).G }
    }

    fn softening(&self) -> f64 {
        unsafe { (*self.raw()).softening }
    }

    fn dt(&self) -> f64 {
        unsafe { (*self.raw()).dt }
    }

    fn n_active(&self) -> i32 {
        unsafe { (*self.raw()).N_active }
    }

    fn testparticle_type(&self) -> i32 {
        unsafe { (*self.raw()).testparticle_type }
    }

    fn testparticle_hidewarnings(&self) -> bool {
        unsafe { (*self.raw()).testparticle_hidewarnings != 0 }
    }

    fn opening_angle2(&self) -> f64 {
        unsafe { (*self.raw()).opening_angle2 }
    }

    fn exact_finish_time(&self) -> bool {
        unsafe { (*self.raw()).exact_finish_time != 0 }
    }

    fn boundary(&self) -> Option<Boundary> {
        unsafe { Boundary::from_raw((*self.raw()).boundary) }
    }

    fn gravity(&self) -> Option<Gravity> {
        unsafe { Gravity::from_raw((*self.raw()).gravity) }
    }

    fn collision(&self) -> Option<Collision> {
        unsafe { Collision::from_raw((*self.raw()).collision) }
    }

    fn force_is_velocity_dependent(&self) -> bool {
        unsafe { (*self.raw()).force_is_velocity_dependent != 0 }
    }

    fn gravity_ignore_terms(&self) -> u32 {
        unsafe { (*self.raw()).gravity_ignore_terms }
    }

    fn exit_max_distance(&self) -> f64 {
        unsafe { (*self.raw()).exit_max_distance }
    }

    fn exit_min_distance(&self) -> f64 {
        unsafe { (*self.raw()).exit_min_distance }
    }

    fn usleep(&self) -> f64 {
        unsafe { (*self.raw()).usleep }
    }

    fn track_energy_offset(&self) -> bool {
        unsafe { (*self.raw()).track_energy_offset != 0 }
    }

    fn collision_resolve_keep_sorted(&self) -> bool {
        unsafe { (*self.raw()).collision_resolve_keep_sorted != 0 }
    }

    fn minimum_collision_velocity(&self) -> f64 {
        unsafe { (*self.raw()).minimum_collision_velocity }
    }

    fn rand_seed(&self) -> u32 {
        unsafe { (*self.raw()).rand_seed }
    }
}

pub trait SimulationSettingsWrite: SimulationSettingsRead + SimulationWrite {
    fn set_g(&mut self, g: f64) -> &mut Self {
        unsafe {
            (*self.raw_mut()).G = g;
        }
        self
    }

    fn set_softening(&mut self, softening: f64) -> Result<&mut Self> {
        if !softening.is_finite() {
            return Err(
                SetError::invalid("softening", format!("must be finite, got {softening}")).into(),
            );
        }
        if softening < 0.0 {
            return Err(
                SetError::invalid("softening", format!("must be >= 0, got {softening}")).into(),
            );
        }
        unsafe {
            (*self.raw_mut()).softening = softening;
        }
        Ok(self)
    }

    fn set_dt(&mut self, dt: f64) -> Result<&mut Self> {
        if !dt.is_finite() {
            return Err(SetError::invalid("dt", format!("must be finite, got {dt}")).into());
        }
        if dt == 0.0 {
            return Err(SetError::invalid("dt", "must be non-zero").into());
        }
        unsafe {
            (*self.raw_mut()).dt = dt;
        }
        Ok(self)
    }

    fn set_testparticle_type(&mut self, testparticle_type: i32) -> Result<&mut Self> {
        if !(0..=1).contains(&testparticle_type) {
            return Err(SetError::invalid(
                "testparticle_type",
                format!("must be 0 or 1, got {testparticle_type}"),
            )
            .into());
        }
        unsafe {
            (*self.raw_mut()).testparticle_type = testparticle_type;
        }
        Ok(self)
    }

    fn set_opening_angle2(&mut self, opening_angle2: f64) -> Result<&mut Self> {
        if !opening_angle2.is_finite() {
            return Err(SetError::invalid(
                "opening_angle2",
                format!("must be finite, got {opening_angle2}"),
            )
            .into());
        }
        if opening_angle2 < 0.0 {
            return Err(SetError::invalid(
                "opening_angle2",
                format!("must be >= 0, got {opening_angle2}"),
            )
            .into());
        }
        unsafe {
            (*self.raw_mut()).opening_angle2 = opening_angle2;
        }
        Ok(self)
    }

    fn set_exact_finish_time(&mut self, exact_finish_time: bool) -> &mut Self {
        unsafe {
            (*self.raw_mut()).exact_finish_time = if exact_finish_time { 1 } else { 0 };
        }
        self
    }

    fn set_n_active(&mut self, n_active: i32) -> Result<&mut Self> {
        let n_real = unsafe { i64::from((*self.raw()).N) - i64::from((*self.raw()).N_var) };
        if n_real < 0 {
            return Err(SetError::invalid(
                "n_active",
                format!("internal state invalid: N - N_var < 0 ({n_real})"),
            )
            .into());
        }
        if n_active != -1 {
            let n_active_i64 = i64::from(n_active);
            if n_active_i64 < 0 || n_active_i64 > n_real {
                return Err(SetError::invalid(
                    "n_active",
                    format!("must be -1 or in [0, {n_real}], got {n_active}"),
                )
                .into());
            }
        }
        unsafe {
            (*self.raw_mut()).N_active = n_active;
        }
        Ok(self)
    }

    fn set_boundary(&mut self, boundary: Boundary) -> &mut Self {
        unsafe {
            (*self.raw_mut()).boundary = boundary.into();
        }
        self
    }

    fn set_gravity(&mut self, gravity: Gravity) -> &mut Self {
        unsafe {
            (*self.raw_mut()).gravity = gravity.into();
        }
        self
    }

    fn set_collision(&mut self, collision: Collision) -> &mut Self {
        unsafe {
            (*self.raw_mut()).collision = collision.into();
        }
        self
    }

    fn set_force_is_velocity_dependent(&mut self, velocity_dependent: bool) -> &mut Self {
        unsafe {
            (*self.raw_mut()).force_is_velocity_dependent = if velocity_dependent { 1 } else { 0 };
        }
        self
    }

    fn set_gravity_ignore_terms(&mut self, gravity_ignore_terms: u32) -> Result<&mut Self> {
        if gravity_ignore_terms > 2 {
            return Err(SetError::invalid(
                "gravity_ignore_terms",
                format!("must be 0, 1, or 2, got {gravity_ignore_terms}"),
            )
            .into());
        }
        unsafe {
            (*self.raw_mut()).gravity_ignore_terms = gravity_ignore_terms;
        }
        Ok(self)
    }

    fn set_exit_max_distance(&mut self, exit_max_distance: f64) -> Result<&mut Self> {
        if !exit_max_distance.is_finite() {
            return Err(SetError::invalid(
                "exit_max_distance",
                format!("must be finite, got {exit_max_distance}"),
            )
            .into());
        }
        if exit_max_distance < 0.0 {
            return Err(SetError::invalid(
                "exit_max_distance",
                format!("must be >= 0, got {exit_max_distance}"),
            )
            .into());
        }
        unsafe {
            (*self.raw_mut()).exit_max_distance = exit_max_distance;
        }
        Ok(self)
    }

    fn set_exit_min_distance(&mut self, exit_min_distance: f64) -> Result<&mut Self> {
        if !exit_min_distance.is_finite() {
            return Err(SetError::invalid(
                "exit_min_distance",
                format!("must be finite, got {exit_min_distance}"),
            )
            .into());
        }
        if exit_min_distance < 0.0 {
            return Err(SetError::invalid(
                "exit_min_distance",
                format!("must be >= 0, got {exit_min_distance}"),
            )
            .into());
        }
        unsafe {
            (*self.raw_mut()).exit_min_distance = exit_min_distance;
        }
        Ok(self)
    }

    fn set_usleep(&mut self, usleep: f64) -> Result<&mut Self> {
        if !usleep.is_finite() {
            return Err(
                SetError::invalid("usleep", format!("must be finite, got {usleep}")).into(),
            );
        }
        if usleep < 0.0 {
            return Err(SetError::invalid("usleep", format!("must be >= 0, got {usleep}")).into());
        }
        unsafe {
            (*self.raw_mut()).usleep = usleep;
        }
        Ok(self)
    }

    fn set_track_energy_offset(&mut self, track_energy_offset: bool) -> &mut Self {
        unsafe {
            (*self.raw_mut()).track_energy_offset = if track_energy_offset { 1 } else { 0 };
        }
        self
    }

    fn set_collision_resolve_keep_sorted(&mut self, keep_sorted: bool) -> &mut Self {
        unsafe {
            (*self.raw_mut()).collision_resolve_keep_sorted = if keep_sorted { 1 } else { 0 };
        }
        self
    }

    fn set_minimum_collision_velocity(
        &mut self,
        minimum_collision_velocity: f64,
    ) -> Result<&mut Self> {
        if !minimum_collision_velocity.is_finite() {
            return Err(SetError::invalid(
                "minimum_collision_velocity",
                format!("must be finite, got {minimum_collision_velocity}"),
            )
            .into());
        }
        if minimum_collision_velocity < 0.0 {
            return Err(SetError::invalid(
                "minimum_collision_velocity",
                format!("must be >= 0, got {minimum_collision_velocity}"),
            )
            .into());
        }
        unsafe {
            (*self.raw_mut()).minimum_collision_velocity = minimum_collision_velocity;
        }
        Ok(self)
    }

    fn set_rand_seed(&mut self, rand_seed: u32) -> &mut Self {
        unsafe {
            (*self.raw_mut()).rand_seed = rand_seed;
        }
        self
    }
}

impl<T: SimulationRead + ?Sized> SimulationSettingsRead for T {}
impl<T: SimulationWrite + ?Sized> SimulationSettingsWrite for T {}

#[cfg(test)]
mod tests {
    use super::{Boundary, Collision, Gravity, SimulationSettingsRead, SimulationSettingsWrite};
    use crate::simulation::Simulation;
    use rebound_bind as rb;

    #[test]
    fn simulation_setting_roundtrips_use_raw_bindgen_types() {
        let mut sim = Simulation::new();
        sim.set_boundary(Boundary::Periodic)
            .set_gravity(Gravity::Trace)
            .set_collision(Collision::LineTree);

        assert_eq!(sim.boundary(), Some(Boundary::Periodic));
        assert_eq!(sim.gravity(), Some(Gravity::Trace));
        assert_eq!(sim.collision(), Some(Collision::LineTree));

        assert_eq!(
            rb::reb_simulation__bindgen_ty_3::from(Boundary::Periodic),
            rb::reb_simulation_REB_BOUNDARY_PERIODIC
        );
        assert_eq!(
            rb::reb_simulation__bindgen_ty_4::from(Gravity::Trace),
            rb::reb_simulation_REB_GRAVITY_TRACE
        );
        assert_eq!(
            rb::reb_simulation__bindgen_ty_1::from(Collision::LineTree),
            rb::reb_simulation_REB_COLLISION_LINETREE
        );
    }
}

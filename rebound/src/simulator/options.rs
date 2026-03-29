use crate::error::{Result, SetError};

use super::Simulation;
use rebound_bind as rb;

// TODO: Confirm that the verification boundaries are accurate.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Collision {
    None = rb::reb_simulation_REB_COLLISION_NONE,
    Direct = rb::reb_simulation_REB_COLLISION_DIRECT,
    Tree = rb::reb_simulation_REB_COLLISION_TREE,
    Line = rb::reb_simulation_REB_COLLISION_LINE,
    LineTree = rb::reb_simulation_REB_COLLISION_LINETREE,
}

impl From<Collision> for rb::reb_simulation__bindgen_ty_1 {
    fn from(value: Collision) -> Self {
        value as Self
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
#[repr(u32)]
pub enum Boundary {
    None = rb::reb_simulation_REB_BOUNDARY_NONE,
    Open = rb::reb_simulation_REB_BOUNDARY_OPEN,
    Periodic = rb::reb_simulation_REB_BOUNDARY_PERIODIC,
    Shear = rb::reb_simulation_REB_BOUNDARY_SHEAR,
}

impl From<Boundary> for rb::reb_simulation__bindgen_ty_3 {
    fn from(value: Boundary) -> Self {
        value as Self
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
#[repr(u32)]
pub enum Gravity {
    None = rb::reb_simulation_REB_GRAVITY_NONE,
    Basic = rb::reb_simulation_REB_GRAVITY_BASIC,
    Compensated = rb::reb_simulation_REB_GRAVITY_COMPENSATED,
    Tree = rb::reb_simulation_REB_GRAVITY_TREE,
    Mercurius = rb::reb_simulation_REB_GRAVITY_MERCURIUS,
    Jacobi = rb::reb_simulation_REB_GRAVITY_JACOBI,
    Trace = rb::reb_simulation_REB_GRAVITY_TRACE,
}

impl From<Gravity> for rb::reb_simulation__bindgen_ty_4 {
    fn from(value: Gravity) -> Self {
        value as Self
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

impl Simulation {
    pub fn t(&self) -> f64 {
        unsafe { (*self.inner).t }
    }

    pub fn g(&self) -> f64 {
        unsafe { (*self.inner).G }
    }

    pub fn softening(&self) -> f64 {
        unsafe { (*self.inner).softening }
    }

    pub fn dt(&self) -> f64 {
        unsafe { (*self.inner).dt }
    }

    pub fn dt_last_done(&self) -> f64 {
        unsafe { (*self.inner).dt_last_done }
    }

    pub fn steps_done(&self) -> u64 {
        unsafe { (*self.inner).steps_done }
    }

    pub fn n(&self) -> usize {
        unsafe { (*self.inner).N as usize }
    }

    pub fn n_var(&self) -> i32 {
        unsafe { (*self.inner).N_var }
    }

    pub fn n_var_config(&self) -> usize {
        unsafe { (*self.inner).N_var_config as usize }
    }

    pub fn n_active(&self) -> i32 {
        unsafe { (*self.inner).N_active }
    }

    pub fn testparticle_type(&self) -> i32 {
        unsafe { (*self.inner).testparticle_type }
    }

    pub fn testparticle_hidewarnings(&self) -> bool {
        unsafe { (*self.inner).testparticle_hidewarnings != 0 }
    }

    pub fn opening_angle2(&self) -> f64 {
        unsafe { (*self.inner).opening_angle2 }
    }

    pub fn exact_finish_time(&self) -> bool {
        unsafe { (*self.inner).exact_finish_time != 0 }
    }

    pub fn status(&self) -> rb::REB_STATUS {
        unsafe { (*self.inner).status }
    }

    pub fn boundary(&self) -> Option<Boundary> {
        unsafe { Boundary::from_raw((*self.inner).boundary) }
    }

    pub fn gravity(&self) -> Option<Gravity> {
        unsafe { Gravity::from_raw((*self.inner).gravity) }
    }

    pub fn collision(&self) -> Option<Collision> {
        unsafe { Collision::from_raw((*self.inner).collision) }
    }

    pub fn force_is_velocity_dependent(&self) -> bool {
        unsafe { (*self.inner).force_is_velocity_dependent != 0 }
    }

    pub fn gravity_ignore_terms(&self) -> u32 {
        unsafe { (*self.inner).gravity_ignore_terms }
    }

    pub fn output_timing_last(&self) -> f64 {
        unsafe { (*self.inner).output_timing_last }
    }

    pub fn exit_max_distance(&self) -> f64 {
        unsafe { (*self.inner).exit_max_distance }
    }

    pub fn exit_min_distance(&self) -> f64 {
        unsafe { (*self.inner).exit_min_distance }
    }

    pub fn usleep(&self) -> f64 {
        unsafe { (*self.inner).usleep }
    }

    pub fn track_energy_offset(&self) -> bool {
        unsafe { (*self.inner).track_energy_offset != 0 }
    }

    pub fn energy_offset(&self) -> f64 {
        unsafe { (*self.inner).energy_offset }
    }

    pub fn walltime(&self) -> f64 {
        unsafe { (*self.inner).walltime }
    }

    pub fn walltime_last_step(&self) -> f64 {
        unsafe { (*self.inner).walltime_last_step }
    }

    pub fn walltime_last_steps(&self) -> f64 {
        unsafe { (*self.inner).walltime_last_steps }
    }

    pub fn walltime_last_steps_sum(&self) -> f64 {
        unsafe { (*self.inner).walltime_last_steps_sum }
    }

    pub fn walltime_last_steps_n(&self) -> i32 {
        unsafe { (*self.inner).walltime_last_steps_N }
    }

    pub fn collision_resolve_keep_sorted(&self) -> bool {
        unsafe { (*self.inner).collision_resolve_keep_sorted != 0 }
    }

    pub fn collisions_n(&self) -> usize {
        unsafe { (*self.inner).collisions_N as usize }
    }

    pub fn collisions_log_n(&self) -> i64 {
        unsafe { (*self.inner).collisions_log_n }
    }

    pub fn minimum_collision_velocity(&self) -> f64 {
        unsafe { (*self.inner).minimum_collision_velocity }
    }

    pub fn collisions_plog(&self) -> f64 {
        unsafe { (*self.inner).collisions_plog }
    }

    pub fn rand_seed(&self) -> u32 {
        unsafe { (*self.inner).rand_seed }
    }

    pub fn set_g(self, g: f64) -> Self {
        unsafe {
            (*self.inner).G = g;
        }
        self
    }

    pub fn set_softening(self, softening: f64) -> Result<Self> {
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
            (*self.inner).softening = softening;
        }
        Ok(self)
    }

    pub fn set_dt(self, dt: f64) -> Result<Self> {
        if !dt.is_finite() {
            return Err(SetError::invalid("dt", format!("must be finite, got {dt}")).into());
        }
        if dt == 0.0 {
            return Err(SetError::invalid("dt", "must be non-zero").into());
        }
        unsafe {
            (*self.inner).dt = dt;
        }
        Ok(self)
    }

    pub fn set_testparticle_type(self, testparticle_type: i32) -> Result<Self> {
        if !(0..=1).contains(&testparticle_type) {
            return Err(SetError::invalid(
                "testparticle_type",
                format!("must be 0 or 1, got {testparticle_type}"),
            )
            .into());
        }
        unsafe {
            (*self.inner).testparticle_type = testparticle_type;
        }
        Ok(self)
    }

    pub fn set_opening_angle2(self, opening_angle2: f64) -> Result<Self> {
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
            (*self.inner).opening_angle2 = opening_angle2;
        }
        Ok(self)
    }

    pub fn set_exact_finish_time(self, exact_finish_time: bool) -> Self {
        unsafe {
            (*self.inner).exact_finish_time = if exact_finish_time { 1 } else { 0 };
        }
        self
    }

    pub fn set_n_active(self, n_active: i32) -> Result<Self> {
        let n_real = unsafe { i64::from((*self.inner).N) - i64::from((*self.inner).N_var) };
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
            (*self.inner).N_active = n_active;
        }
        Ok(self)
    }

    pub fn set_boundary(self, boundary: Boundary) -> Self {
        unsafe {
            (*self.inner).boundary = boundary.into();
        }
        self
    }

    pub fn set_gravity(self, gravity: Gravity) -> Self {
        unsafe {
            (*self.inner).gravity = gravity.into();
        }
        self
    }

    pub fn set_collision(self, collision: Collision) -> Self {
        unsafe {
            (*self.inner).collision = collision.into();
        }
        self
    }

    pub fn set_force_is_velocity_dependent(self, velocity_dependent: bool) -> Self {
        unsafe {
            (*self.inner).force_is_velocity_dependent = if velocity_dependent { 1 } else { 0 };
        }
        self
    }

    pub fn set_gravity_ignore_terms(self, gravity_ignore_terms: u32) -> Result<Self> {
        if gravity_ignore_terms > 2 {
            return Err(SetError::invalid(
                "gravity_ignore_terms",
                format!("must be 0, 1, or 2, got {gravity_ignore_terms}"),
            )
            .into());
        }
        unsafe {
            (*self.inner).gravity_ignore_terms = gravity_ignore_terms;
        }
        Ok(self)
    }

    pub fn set_exit_max_distance(self, exit_max_distance: f64) -> Result<Self> {
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
            (*self.inner).exit_max_distance = exit_max_distance;
        }
        Ok(self)
    }

    pub fn set_exit_min_distance(self, exit_min_distance: f64) -> Result<Self> {
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
            (*self.inner).exit_min_distance = exit_min_distance;
        }
        Ok(self)
    }

    pub fn set_usleep(self, usleep: f64) -> Result<Self> {
        if !usleep.is_finite() {
            return Err(
                SetError::invalid("usleep", format!("must be finite, got {usleep}")).into(),
            );
        }
        if usleep < 0.0 {
            return Err(SetError::invalid("usleep", format!("must be >= 0, got {usleep}")).into());
        }
        unsafe {
            (*self.inner).usleep = usleep;
        }
        Ok(self)
    }

    pub fn set_track_energy_offset(self, track_energy_offset: bool) -> Self {
        unsafe {
            (*self.inner).track_energy_offset = if track_energy_offset { 1 } else { 0 };
        }
        self
    }

    pub fn set_collision_resolve_keep_sorted(self, keep_sorted: bool) -> Self {
        unsafe {
            (*self.inner).collision_resolve_keep_sorted = if keep_sorted { 1 } else { 0 };
        }
        self
    }

    pub fn set_minimum_collision_velocity(self, minimum_collision_velocity: f64) -> Result<Self> {
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
            (*self.inner).minimum_collision_velocity = minimum_collision_velocity;
        }
        Ok(self)
    }

    pub fn set_rand_seed(self, rand_seed: u32) -> Self {
        unsafe {
            (*self.inner).rand_seed = rand_seed;
        }
        self
    }
}

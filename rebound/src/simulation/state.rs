use super::Simulation;
use rebound_bind as rb;

impl Simulation {
    pub fn t(&self) -> f64 {
        unsafe { (*self.inner).t }
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

    pub fn status(&self) -> rb::REB_STATUS {
        unsafe { (*self.inner).status }
    }

    pub fn output_timing_last(&self) -> f64 {
        unsafe { (*self.inner).output_timing_last }
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

    pub fn collisions_n(&self) -> usize {
        unsafe { (*self.inner).collisions_N as usize }
    }

    pub fn collisions_log_n(&self) -> i64 {
        unsafe { (*self.inner).collisions_log_n }
    }

    pub fn collisions_plog(&self) -> f64 {
        unsafe { (*self.inner).collisions_plog }
    }
}

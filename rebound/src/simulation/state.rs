use super::SimulationRead;
use rebound_bind as rb;

pub trait SimulationStateRead: SimulationRead {
    fn t(&self) -> f64 {
        unsafe { (*self.raw()).t }
    }

    fn dt_last_done(&self) -> f64 {
        unsafe { (*self.raw()).dt_last_done }
    }

    fn steps_done(&self) -> u64 {
        unsafe { (*self.raw()).steps_done }
    }

    fn n(&self) -> usize {
        unsafe { (*self.raw()).N as usize }
    }

    fn n_var(&self) -> i32 {
        unsafe { (*self.raw()).N_var }
    }

    fn n_var_config(&self) -> usize {
        unsafe { (*self.raw()).N_var_config as usize }
    }

    fn status(&self) -> rb::REB_STATUS {
        unsafe { (*self.raw()).status }
    }

    fn output_timing_last(&self) -> f64 {
        unsafe { (*self.raw()).output_timing_last }
    }

    fn energy_offset(&self) -> f64 {
        unsafe { (*self.raw()).energy_offset }
    }

    fn walltime(&self) -> f64 {
        unsafe { (*self.raw()).walltime }
    }

    fn walltime_last_step(&self) -> f64 {
        unsafe { (*self.raw()).walltime_last_step }
    }

    fn walltime_last_steps(&self) -> f64 {
        unsafe { (*self.raw()).walltime_last_steps }
    }

    fn walltime_last_steps_sum(&self) -> f64 {
        unsafe { (*self.raw()).walltime_last_steps_sum }
    }

    fn walltime_last_steps_n(&self) -> i32 {
        unsafe { (*self.raw()).walltime_last_steps_N }
    }

    fn collisions_n(&self) -> usize {
        unsafe { (*self.raw()).collisions_N as usize }
    }

    fn collisions_log_n(&self) -> i64 {
        unsafe { (*self.raw()).collisions_log_n }
    }

    fn collisions_plog(&self) -> f64 {
        unsafe { (*self.raw()).collisions_plog }
    }
}

impl<T: SimulationRead + ?Sized> SimulationStateRead for T {}

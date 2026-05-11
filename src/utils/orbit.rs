use crate::bind;

/// Returns `angle` wrapped into the interval `[0, 2π)`.
///
/// # Arguments
///
/// * `angle` - The angle to wrap.
///
/// # Returns
///
/// The wrapped angle.
pub fn mod_2pi(angle: f64) -> f64 {
    unsafe { bind::reb_mod2pi(angle) }
}

/// Converts mean anomaly to eccentric anomaly.
///
/// # Arguments
///
/// * `eccentricity` - The eccentricity of the orbit.
/// * `mean_anomaly` - The mean anomaly to convert.
///
/// # Returns
///
/// The eccentric anomaly.
pub fn mean_to_eccentric_anomaly(eccentricity: f64, mean_anomaly: f64) -> f64 {
    unsafe { bind::reb_M_to_E(eccentricity, mean_anomaly) }
}

/// Converts mean anomaly to true anomaly.
///
/// # Arguments
///
/// * `eccentricity` - The eccentricity of the orbit.
/// * `mean_anomaly` - The mean anomaly to convert.
///
/// # Returns
///
/// The true anomaly.
pub fn mean_to_true_anomaly(eccentricity: f64, mean_anomaly: f64) -> f64 {
    unsafe { bind::reb_M_to_f(eccentricity, mean_anomaly) }
}

/// Converts eccentric anomaly to true anomaly.
///
/// # Arguments
///
/// * `eccentricity` - The eccentricity of the orbit.
/// * `eccentric_anomaly` - The eccentric anomaly to convert.
///
/// # Returns
///
/// The true anomaly.
pub fn eccentric_to_true_anomaly(eccentricity: f64, eccentric_anomaly: f64) -> f64 {
    unsafe { bind::reb_E_to_f(eccentricity, eccentric_anomaly) }
}

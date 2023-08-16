//! This module organizes the types needed to represent the results of the simulation.

#[derive(Debug, Clone, Copy)]
pub struct SimResult {
    /// Power demand in case all charging stations are active at the same time.
    pub theoretical_max_kW: f32,
    /// The station's accumulated energy consumption of a simulated year.
    pub consumption_year_kWh: f32,
    /// Actually demanded max power output of the station.
    pub actual_max_kW: f32,
}

impl SimResult {
    pub fn init_zero() -> Self {
        Self {
            theoretical_max_kW: 0.0,
            consumption_year_kWh: 0.0,
            actual_max_kW: 0.0,
        }
    }
}

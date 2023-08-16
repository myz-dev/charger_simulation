use crate::probabilities::{ArrivalProbabilities, ChargeProbabilities};

const DEFAULT_CHARGE_POINTS: u32 = 20;
/// Contains all parameters for the charging station simulation.
#[derive(Debug, Clone, Copy)]
pub struct Input {
    /// Number of total parking lots with an EV charger.
    pub charge_points: u32,
    pub arrival_probabilities: ArrivalProbabilities,
    pub charge_probabilities: ChargeProbabilities,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            charge_points: DEFAULT_CHARGE_POINTS,
            arrival_probabilities: crate::probabilities::ARRIVAL,
            charge_probabilities: crate::probabilities::DEMAND,
        }
    }
}

// Boilerplate for easy access to members.

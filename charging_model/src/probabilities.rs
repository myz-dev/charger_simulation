//! This module contains the distributed probability of EVs arriving at a parking lot and their charging needs.

use log::error;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

/// Chance of arrival in percent for hourly time slots of a day.
pub type ArrivalProbabilities = [f32; 24];
/// Chance of charging `n` km worth of electricity.
pub type ChargeProbabilities = [ChargingNeed; 9];

/// Probability of a EVs arriving at a parking lot within one tick of the simulation.
/// This probability is used on every tick of the simulation on each parking lot to determine wether or not a new EV
/// arrives.
/// The array maps all 24 hours of a day, starting at 00:00 at index `ARRIVAL[0]` and
/// ending at 23:59 at index `ARRIVAL[23]`.
pub(crate) const ARRIVAL: ArrivalProbabilities = [
    0.0094, 0.0094, 0.0094, 0.0094, 0.0094, 0.0094, 0.0094, 0.0094, 0.0283, 0.0283, 0.0566, 0.0566,
    0.0566, 0.0755, 0.0755, 0.0755, 0.1038, 0.1038, 0.1038, 0.0472, 0.0472, 0.0472, 0.0094, 0.0094,
];

/// Probability of an EV wanting to charge its batteries at the station and how much it plans to charge.
/// The demand is expressed as the expected range that the EV will be able to travel with the charge in km.
pub(crate) const DEMAND: ChargeProbabilities = [
    ChargingNeed::new(0.3431, 0.0),
    ChargingNeed::new(0.049, 5.0),
    ChargingNeed::new(0.098, 10.0),
    ChargingNeed::new(0.1176, 20.0),
    ChargingNeed::new(0.0882, 30.0),
    ChargingNeed::new(0.1176, 50.0),
    ChargingNeed::new(0.1078, 100.0),
    ChargingNeed::new(0.049, 200.0),
    ChargingNeed::new(0.0294, 200.0),
];

#[derive(Debug, Clone, Copy)]
pub struct ChargingNeed {
    /// Probability in percent.
    pub prob: f32,
    /// Range that should be charged.
    pub charge_km: f32,
}

impl ChargingNeed {
    pub const fn new(prob: f32, charge_km: f32) -> Self {
        Self { prob, charge_km }
    }
}

pub(crate) trait CalculateMean<T> {
    fn to_mean(&self) -> f32;
}

impl CalculateMean<ChargeProbabilities> for ChargeProbabilities {
    /// Calculates the mean value of the charged distance for the given distribution.
    fn to_mean(&self) -> f32 {
        self.iter().map(|d| d.charge_km * d.prob).sum()
    }
}

pub(crate) trait GetWeightedDemand<T> {
    fn get_weighted_demand(&self) -> f32;
}

impl GetWeightedDemand<ChargeProbabilities> for ChargeProbabilities {
    /// Generates a random number and checks with which sector in the given demand probability distribution the
    /// generated value corresponds.
    fn get_weighted_demand(&self) -> f32 {
        let mut rng = thread_rng();
        let gen = Uniform::from(0.0..1.0);

        let mut acc = 0.0;
        for sector in self.iter() {
            acc += sector.prob;
            if gen.sample(&mut rng) <= acc {
                return sector.charge_km;
            }
        }
        self.iter()
            .last()
            .unwrap_or_else(|| {
                error!("Ran simulation with empty demand distribution!");
                &ChargingNeed {
                    prob: 0.0,
                    charge_km: 0.0,
                }
            })
            .charge_km
    }
}

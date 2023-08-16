//! This module contains the distributed probability of EVs arriving at a parking lot and their charging needs.

/// Probability of a EVs arriving at a parking lot within one tick of the simulation.
/// This probability is used on every tick of the simulation on each parking lot to determine wether or not a new EV
/// arrives.
/// The array maps all 24 hours of a day, starting at 00:00 at index `ARRIVAL[0]` and
/// ending at 23:59 at index `ARRIVAL[23]`.
pub(crate) const ARRIVAL: [f32; 23] = [
    0.0, 0.0, 0.94, 0.94, 0.94, 0.94, 0.94, 2.83, 2.83, 5.66, 5.66, 5.66, 7.55, 7.55, 7.55, 10.38,
    10.38, 10.38, 4.72, 4.72, 4.72, 0.94, 0.94,
];

/// Probability of an EV wanting to charge its batteries at the station and how much it plans to charge.
/// The demand is expressed as the expected range that the EV will be able to travel with the charge in km.
pub(crate) const DEMAND: [ChargingNeed; 9] = [
    ChargingNeed::new(34.31, 0.0),
    ChargingNeed::new(4.9, 5.0),
    ChargingNeed::new(9.8, 10.0),
    ChargingNeed::new(11.76, 20.0),
    ChargingNeed::new(8.82, 30.0),
    ChargingNeed::new(11.76, 50.0),
    ChargingNeed::new(10.78, 100.0),
    ChargingNeed::new(4.9, 200.0),
    ChargingNeed::new(2.94, 200.0),
];

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChargingNeed {
    /// Probability in percent.
    prob: f32,
    /// Range that should be charged.
    charge_km: f32,
}

impl ChargingNeed {
    pub const fn new(prob: f32, charge_km: f32) -> Self {
        Self { prob, charge_km }
    }
}

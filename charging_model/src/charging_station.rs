//! Logic to handle the distribution/occupancy of charging stations.

use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

use crate::{
    output::SimResult,
    probabilities::{ChargeProbabilities, GetWeightedDemand},
};

/// Power that is used to charge the batteries of the EVs in kW.
pub(crate) const DEFAULT_CHARGING_SPEED: f32 = 11.0;
/// Amount of energy needed to charge one EV with electricity worth 1km of traveling in [kWh/km].
const DEFAULT_ENERGY_PER_1KM: f32 = 0.18;
const TICKS_PER_HOUR: usize = 4;

#[derive(Debug, Clone, Copy)]
pub(crate) enum StationManagerError {
    /// Contains the number of EVs that could not be assigned to a charging station.
    NoEmptyChargingStations(u32),
}

/// Charging stations can only serve one EV at a time. The EV is assumed to leave the charging station in the last
/// tick of its charging period.
#[derive(Debug)]
pub(crate) struct ChargingStation {
    /// Ticks left until the charging station is available again.
    occupied_for_ticks: u32,
    /// Power output of the charging station.
    charging_speed_kW: f32,
}

impl ChargingStation {
    pub(crate) fn new(charging_speed_kW: f32) -> ChargingStation {
        ChargingStation {
            occupied_for_ticks: 0,
            charging_speed_kW,
        }
    }

    /// Calculates the number of ticks that the charging station will be occupied.
    /// Updates the occupied_for_ticks member.
    /// # Note
    /// This function rounds up the number of occupied ticks to the nearest integer.
    pub fn occupy(&mut self, charge_probabilities: &ChargeProbabilities) {
        // hours_needed = (demand / charging speed) * battery_efficiency ~ [km]/[kW] * [kWh]/km
        let demand_km = charge_probabilities.get_weighted_demand();
        let hours_needed = (demand_km / self.charging_speed_kW) * DEFAULT_ENERGY_PER_1KM;
        self.occupied_for_ticks = (hours_needed * TICKS_PER_HOUR as f32).ceil() as u32;
    }

    /// Decrements the occupied_for_ticks member.
    /// If the tick counter is already 0, nothing happens.
    pub fn pass_tick(&mut self) {
        self.occupied_for_ticks = self.occupied_for_ticks.saturating_sub(1);
    }
}

impl Default for ChargingStation {
    fn default() -> Self {
        Self {
            occupied_for_ticks: 0,
            charging_speed_kW: DEFAULT_CHARGING_SPEED,
        }
    }
}

/// Manages the assignment of charging stations to EVs and calculates the energy consumption of the system.
pub(crate) struct StationManager {
    stations: Vec<ChargingStation>,
    pub res: SimResult,
}

impl StationManager {
    /// Creates a new StationManager with the given number of charging stations and calculates the theoretical
    /// maximum power demand of the system.
    pub(crate) fn new(count: usize, charging_speed_kW: f32) -> Self {
        let mut manager = Self {
            stations: (0..count)
                .map(|_| ChargingStation::new(charging_speed_kW))
                .collect(),
            res: SimResult::init_zero(),
        };
        manager.res.theoretical_max_kW = manager.stations.iter().map(|s| s.charging_speed_kW).sum();
        manager
    }

    /// Call every tick to move out fully charged EVs and move in new ones.
    /// # Parameters
    /// - `prob`: The probability of a new EV arriving at a station in percent
    /// - The mean charging demand in km as `demand_km`.
    pub(crate) fn recalculate_tick(
        &mut self,
        arrival_probability: f32,
        charge_probabilities: &ChargeProbabilities,
    ) -> Result<(), StationManagerError> {
        self.remove_charged_evs();
        self.add_new_evs(arrival_probability, charge_probabilities)?;
        self.update_sim_results();
        Ok(())
    }

    /// Simulate a tick passing by decrementing all occupancy counters.
    fn remove_charged_evs(&mut self) {
        for station in self.stations.iter_mut() {
            station.pass_tick();
        }
    }

    /// Fills stations with new EVs according to their needs'.
    /// The number of newly arriving EVs is rounded to the nearest integer.
    fn add_new_evs(
        &mut self,
        arrival_probability: f32,
        charge_probabilities: &ChargeProbabilities,
    ) -> Result<(), StationManagerError> {
        let mut new_evs = self.populate_randomly(arrival_probability);

        for station in self.stations.iter_mut() {
            if station.occupied_for_ticks != 0 {
                continue;
            }

            station.occupy(charge_probabilities);
            new_evs = new_evs.saturating_sub(1);

            if new_evs == 0 {
                break;
            }
        }

        if new_evs != 0 {
            return Err(StationManagerError::NoEmptyChargingStations(new_evs));
        }
        Ok(())
    }

    /// Updates the maximum power demand if necessary and keeps track of the total energy consumption.
    fn update_sim_results(&mut self) {
        let current_power_demand = self
            .stations
            .iter()
            .filter_map(|s| {
                if s.occupied_for_ticks > 0 {
                    Some(s.charging_speed_kW)
                } else {
                    None
                }
            })
            .sum();
        if current_power_demand > self.res.actual_max_kW {
            self.res.actual_max_kW = current_power_demand;
        }
        self.res.consumption_year_kWh += current_power_demand * (1.0 / TICKS_PER_HOUR as f32);
    }

    /// "Rolls the dice" with the given probability to populate each charging slot with a new EV arrival.
    fn populate_randomly(&self, probability: f32) -> u32 {
        let mut new_evs = 0;
        let mut rng = thread_rng();
        let gen = Uniform::from(0.0..1.0);
        for _ in 0..self.stations.len() {
            let rolled = gen.sample(&mut rng);

            if rolled <= probability {
                new_evs += 1;
            }
        }
        new_evs
    }
}

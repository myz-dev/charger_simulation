//! The simulation of the energy consumption of the charging stations depends on the following assumptions:
//! - Temporal resolution is 15 minutes (365d * 24 * 4 = 35040 ticks).
//! - Charging station can charge one EV (electric vehicle) at a time.
//! - EV leaves the station as soon as it is finished charging.
//! - All EVs consume 18kWh for a 100km charge.
//!
//! The simulation calculates:
//! - Total energy consumed in kWh
//! - The theoretical maximum power demand
//! - The actual maximum power demand (= the maximum sum of all charge points power
//!   demands at a given 15-minute interval)
//! - The ratio of actual to maximum power demand ("concurrency factor"

use log::{info, trace, warn};
const DEFAULT_TICK_COUNT: usize = 365 * 24 * 4; //35040 ~ 15min per tick

use crate::{
    charging_station::{
        StationManager, StationManagerError::NoEmptyChargingStations, DEFAULT_CHARGING_SPEED,
    },
    input::Input,
    output::SimResult,
    probabilities::{ArrivalProbabilities, CalculateMean},
};

/// Simulates the energy consumption characteristics of a EV charging station.
/// For now the simulation generates random numbers to get randomized results when calculating values from
/// probability distributions. See [`crate::probabilities::GetWeightedDemand<T>`]
/// for the implementation of the demand-resolver.
pub fn simulate_station(input: Input) -> SimResult {
    trace!("Simulating EV charging station...");
    trace!("Input:\n{:#?}", input);

    let Input {
        charge_points,
        arrival_probabilities,
        charge_probabilities,
    } = input;
    let mut stations = StationManager::new(charge_points as usize, DEFAULT_CHARGING_SPEED);

    let mean_charge = charge_probabilities.to_mean();
    info!("The current distributions mean charge is {mean_charge}km per EV and charge.");

    // Get expected amount of EVs looking for chargers per tick and how much they are expected to charge.
    for tick in 0..DEFAULT_TICK_COUNT {
        let arrival_probability = map_tick_to_arrival_probability(tick, &arrival_probabilities);
        if let Err(e) = stations.recalculate_tick(arrival_probability, &charge_probabilities) {
            match e {
                NoEmptyChargingStations(count) => warn!(
                    "Not enough charging stations to satisfy the demand! EVs not served: {count}"
                ),
            }
        }
    }

    info!(
        "Finishing the simulation. The calculated results are:\n{:#?}.",
        stations.res
    );
    stations.res
}

/// Takes a tick of a year and the number of parking lots and returns the corresponding lot occupancy probability in
/// percent.
/// # Panics
/// This function assumes the passed `prob` slice has a length of 23 and therefore panics if it is shorter than that.
fn map_tick_to_arrival_probability(tick: usize, prob: &ArrivalProbabilities) -> f32 {
    let index = tick % 24;
    prob[index]
}

#[cfg(test)]
mod test {
    use crate::input::Input;
    use test_log::test;

    #[test]
    fn test_sim() {
        let input = Input::default();
        let res = super::simulate_station(input);
        println!("Results are:\n{:#?}", res);
        println!(
            "Concurrency factor: {}",
            res.actual_max_kW / res.theoretical_max_kW
        );
    }
}

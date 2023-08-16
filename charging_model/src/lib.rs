//! This library organizes the code used to simulate the expected energy consumption/ peak power usage
//! of a car charging station.
//! The simulation can be used to explore different system configurations and asses different deployments of charging
//! stations.
#![allow(non_snake_case)]
pub(crate) mod charging_station;
pub mod error;
pub mod input;
pub mod output;
pub mod probabilities;
pub mod sim;

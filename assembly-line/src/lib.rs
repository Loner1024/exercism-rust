// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::env::split_paths;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed > 0 && speed <= 4 {
        speed as f64 * 221 as f64
    } else if speed >= 5 && speed <= 8 {
        speed as f64 * 221 as f64 * 0.9
    } else if speed >= 9 && speed <= 10 {
        speed as f64 * 221 as f64 * 0.77
    } else {
        0.0
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}

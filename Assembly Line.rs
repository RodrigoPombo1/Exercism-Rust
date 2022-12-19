#![allow(unused)]
pub fn production_rate_per_hour(speed: u8) -> f64 {
    (speed as f64) * 221.0 * (success_rate_calculator(speed) as f64)/100.0
}
pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
pub fn success_rate_calculator(speed: u8) -> f64 {
    match speed {
        0 => 0.0,
        1..=4 => 100.0,
        5..=8 => 90.0,
        9..=u8::MAX => 77.0,
    }
}
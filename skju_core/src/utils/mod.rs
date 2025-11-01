use crate::common::SensorConfig;
use serde_json::from_str;
use std::fs::read_to_string;

pub fn get_sensors_from_file(path: &str) -> anyhow::Result<Vec<SensorConfig>> {
    let data = read_to_string(path)?;
    let sensors: Vec<SensorConfig> = from_str(&data)?;
    Ok(sensors)
}

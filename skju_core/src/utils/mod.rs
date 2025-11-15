use crate::common::SensorConfig;
use std::fs::read_to_string;
use std::path::Path;

pub fn get_sensors_from_file<T: AsRef<Path>>(path: T) -> Result<Vec<SensorConfig>, String> {
    let data = read_to_string(path).map_err(|e| e.to_string())?;
    let sensors = data
        .lines()
        .map(|line| line.parse::<SensorConfig>())
        .collect::<Result<Vec<SensorConfig>, String>>()?;

    Ok(sensors)
}

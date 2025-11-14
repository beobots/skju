use crate::common::{Coord, FilterContext, LowPassFilter, SensorConfig, SensorData, SensorOutput};
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

// region: Typed Fields
pub struct WithCoord(Coord);
pub struct NoCoord;

pub struct WithFilter<T: LowPassFilter>(T);
pub struct NoFilter;

pub struct WithCapacity(usize);
pub struct NoCapacity;
// endregion: Typed Fields

pub struct Sensor<T: LowPassFilter> {
    pub id: u64,
    pub name: String,
    pub coord: Coord,
    pub filter: T,
    capacity: usize,
    readings: VecDeque<SensorData>,
}

pub struct SensorBuilder<F, U, C> {
    id: u64,
    name: String,
    coord: U,
    filter: F,
    capacity: C,
    readings: Option<VecDeque<SensorData>>,
}

#[derive(Debug, Clone)]
pub enum SensorBuildError {
    MissingCoord,
    MissingCapacity,
    MissingFilter,
}

impl SensorBuilder<NoFilter, NoCoord, NoCapacity> {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(id: u64, name: &str) -> SensorBuilder<NoFilter, NoCoord, NoCapacity> {
        SensorBuilder {
            id,
            name: name.to_string(),
            coord: NoCoord,
            filter: NoFilter,
            capacity: NoCapacity,
            readings: None,
        }
    }
}

impl<U, C> SensorBuilder<NoFilter, U, C> {
    pub fn filter<T: LowPassFilter>(self, filter: T) -> SensorBuilder<WithFilter<T>, U, C> {
        SensorBuilder {
            id: self.id,
            name: self.name,
            coord: self.coord,
            capacity: self.capacity,
            readings: self.readings,
            filter: WithFilter(filter),
        }
    }
}

impl<F, C> SensorBuilder<F, NoCoord, C> {
    pub fn coord(self, coord: Coord) -> SensorBuilder<F, WithCoord, C> {
        SensorBuilder {
            id: self.id,
            name: self.name,
            coord: WithCoord(coord),
            capacity: self.capacity,
            readings: self.readings,
            filter: self.filter,
        }
    }
}

impl<F, U> SensorBuilder<F, U, NoCapacity> {
    pub fn with_capacity(self, capacity: usize) -> SensorBuilder<F, U, WithCapacity> {
        SensorBuilder {
            id: self.id,
            name: self.name,
            coord: self.coord,
            filter: self.filter,
            capacity: WithCapacity(capacity),
            readings: Some(VecDeque::with_capacity(capacity)),
        }
    }
}

impl<T: LowPassFilter> SensorBuilder<WithFilter<T>, WithCoord, WithCapacity> {
    pub fn build(self) -> Sensor<T> {
        Sensor {
            id: self.id,
            name: self.name,
            coord: self.coord.0,
            filter: self.filter.0,
            capacity: self.capacity.0,
            readings: self.readings.unwrap_or_default(),
        }
    }
}

impl<T: LowPassFilter> Sensor<T> {
    pub fn is_empty(&self) -> bool {
        self.readings.is_empty()
    }

    pub fn write(&mut self, value: f64, timestamp: u128) {
        let context = FilterContext {
            timestamp,
            readings: &self.readings,
            raw_value: value,
            capacity: self.capacity,
        };

        let data = SensorData {
            value: self.filter.apply(&context),
            timestamp,
        };

        if self.readings.len() == self.capacity {
            self.readings.pop_front();
        }

        self.readings.push_back(data);
    }

    pub fn get_latest(&mut self) -> Option<SensorOutput> {
        let latest = self.readings.back()?;
        let output = SensorOutput {
            sensor_id: self.id,
            sensor_name: self.name.clone(),
            sensor_coord: Coord { x: self.coord.x, y: self.coord.y },
            value: latest.value,
            timestamp: latest.timestamp,
        };

        Some(output)
    }

    pub fn read(&mut self) -> Option<SensorData> {
        self.readings.pop_front()
    }
}

impl Display for SensorData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{};{}", self.value, self.timestamp)
    }
}

impl FromStr for SensorData {
    type Err = String;

    fn from_str(data_str: &str) -> Result<Self, Self::Err> {
        let mut split = data_str.split(';');

        let value: f64 = split
            .next()
            .ok_or("Missing value")?
            .parse()
            .map_err(|_| "Unable to parse value")?;

        let timestamp: u128 = split
            .next()
            .ok_or("Missing timestamp")?
            .parse()
            .map_err(|_| "Unable to parse timestamp")?;

        Ok(SensorData { value, timestamp })
    }
}

impl Display for SensorConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{};{};{};{}", self.id, self.name, self.coord.x, self.coord.y)
    }
}

impl FromStr for SensorConfig {
    type Err = String;

    fn from_str(config_str: &str) -> Result<Self, Self::Err> {
        let mut split = config_str.split(';');

        let id: u64 = split
            .next()
            .ok_or("Missing id")?
            .parse()
            .map_err(|_| "Unable to parse id")?;

        let name: String = split
            .next()
            .ok_or("Missing sensor name")?
            .to_string();

        let coord_x: f32 = split
            .next()
            .ok_or("Missing coord x")?
            .parse()
            .map_err(|_| "Unable to parse x coord")?;

        let coord_y: f32 = split
            .next()
            .ok_or("Missing coord y")?
            .parse()
            .map_err(|_| "Unable to parse y coord")?;
        let coord = Coord { x: coord_x, y: coord_y };

        Ok(SensorConfig { id, name, coord })
    }
}

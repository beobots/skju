use crate::common::{Coord, FilterContext, LowPassFilter, SensorData, SensorOutput};
use std::collections::VecDeque;

pub struct Sensor<T: LowPassFilter> {
    pub id: u64,
    pub name: String,
    pub coord: Coord,
    pub filter: T,
    capacity: usize,
    readings: VecDeque<SensorData>,
}

pub struct SensorBuilder<T: LowPassFilter> {
    id: u64,
    name: String,
    coord: Option<Coord>,
    filter: Option<T>,
    capacity: Option<usize>,
    readings: Option<VecDeque<SensorData>>,
}

#[derive(Debug, Clone)]
pub enum SensorBuildError {
    MissingCoord,
    MissingCapacity,
    MissingFilter,
}

unsafe impl<T: LowPassFilter> Send for Sensor<T> {}
unsafe impl<T: LowPassFilter> Sync for Sensor<T> {}

impl<T: LowPassFilter> SensorBuilder<T> {
    pub fn coord(mut self, coord: Coord) -> Self {
        self.coord = Some(coord);
        self
    }

    pub fn filter(mut self, filter: T) -> Self {
        self.filter = Some(filter);
        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = Some(capacity);
        self.readings = Some(VecDeque::with_capacity(capacity));
        self
    }

    pub fn build(self) -> Result<Sensor<T>, SensorBuildError> {
        let sensor = Sensor {
            id: self.id,
            name: self.name,
            coord: self.coord.ok_or(SensorBuildError::MissingCoord)?,
            filter: self.filter.ok_or(SensorBuildError::MissingFilter)?,
            capacity: self.capacity.ok_or(SensorBuildError::MissingCapacity)?,
            readings: self.readings.unwrap_or_default(),
        };

        Ok(sensor)
    }
}

impl<T: LowPassFilter> Sensor<T> {
    pub fn new(id: u64, name: &str) -> SensorBuilder<T> {
        SensorBuilder {
            id,
            name: name.to_string(),
            coord: None,
            filter: None,
            capacity: None,
            readings: None,
        }
    }

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

use crate::common::{Coord, FilterContext, LowPassFilter, SensorData, SensorOutput};
use std::collections::VecDeque;

pub struct Sensor<T: LowPassFilter> {
    pub id: u64,
    pub name: String,
    pub coords: Coord,
    pub filter: T,
    capacity: usize,
    readings: VecDeque<SensorData>,
}

unsafe impl<T: LowPassFilter> Send for Sensor<T> {}
unsafe impl<T: LowPassFilter> Sync for Sensor<T> {}

impl<T: LowPassFilter> Sensor<T> {
    pub fn new(id: u64, name: &str, coords: Coord, capacity: usize, filter: T) -> Self {
        Self {
            id,
            capacity,
            coords,
            filter,
            name: name.to_string(),
            readings: VecDeque::with_capacity(capacity),
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
            sensor_coord: Coord { x: self.coords.x, y: self.coords.y },
            value: latest.value,
            timestamp: latest.timestamp,
        };

        Some(output)
    }

    pub fn read(&mut self) -> Option<SensorData> {
        self.readings.pop_front()
    }
}

use crate::common::{Coord, FilterContext, LowPassFilter, SensorData};
use std::collections::VecDeque;
use std::time::SystemTime;

pub struct Sensor<T: LowPassFilter> {
    pub name: String,
    pub coords: Coord,
    pub filter: T,
    capacity: usize,
    readings: VecDeque<SensorData>,
}

impl<T: LowPassFilter> Sensor<T> {
    pub fn new(name: &str, coords: Coord, capacity: usize, filter: T) -> Self {
        Self {
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

    pub fn write(&mut self, value: f64, timestamp: SystemTime) {
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

    pub fn read(&mut self) -> Option<SensorData> {
        self.readings.pop_front()
    }
}

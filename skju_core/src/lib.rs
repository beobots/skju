use std::collections::VecDeque;
use std::time::SystemTime;

pub struct Sensor {
    pub name: String,
    pub coords: Coord,
    limit: usize,
    readings: VecDeque<SensorData>,
}

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct SensorData {
    pub value: f64,
    pub timestamp: SystemTime,
}

impl Sensor {
    pub fn new(name: &str, coords: Coord, limit: usize) -> Self {
        Self {
            limit,
            coords,
            name: name.to_string(),
            readings: VecDeque::with_capacity(limit),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.readings.is_empty()
    }

    pub fn write(&mut self, data: SensorData) {
        if self.readings.len() == self.limit {
            self.readings.pop_front();
        }

        self.readings.push_back(data);
    }

    pub fn read(&mut self) -> Option<SensorData> {
        self.readings.pop_front()
    }
}

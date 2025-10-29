use std::collections::VecDeque;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub struct SensorData {
    pub value: f64,
    pub timestamp: SystemTime,
}

pub struct FilterContext<'a> {
    pub readings: &'a VecDeque<SensorData>,
    pub raw_value: f64,
    pub timestamp: SystemTime,
    pub capacity: usize,
}

pub trait LowPassFilter {
    fn apply(&mut self, context: &FilterContext) -> f64;
}

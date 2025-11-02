use crate::common::{FilterContext, LowPassFilter};

pub struct MultiPoleExponentialLowPass {
    pub stages: u8,
    pub smoothing: f32,
    prev_stages: Vec<f64>,
}

impl MultiPoleExponentialLowPass {
    pub fn new(stages: u8, smoothing: f32) -> MultiPoleExponentialLowPass {
        MultiPoleExponentialLowPass { stages, smoothing, prev_stages: vec![] }
    }
}

impl LowPassFilter for MultiPoleExponentialLowPass {
    fn apply(&mut self, context: &FilterContext) -> f64 {
        if self.prev_stages.is_empty() {
            let last_reading = context
                .readings
                .back()
                .map(|r| r.value)
                .unwrap_or(context.raw_value);

            self.prev_stages = vec![last_reading; self.stages as usize];
        }

        let smoothing = self.smoothing as f64;
        let mut new_stages = vec![];
        let prev_stages = &self.prev_stages;
        let mut result = context.raw_value;

        for i in 0..prev_stages.len() {
            let value = if i == 0 { context.raw_value } else { new_stages[i - 1] };
            let new_stage_value = prev_stages[i] + smoothing * (value - prev_stages[i]);

            new_stages.push(new_stage_value);
            result = new_stage_value;
        }

        result
    }
}

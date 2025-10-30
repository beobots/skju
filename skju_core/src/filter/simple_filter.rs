use crate::common::{FilterContext, LowPassFilter};

pub struct SimpleFilter {
    pub smoothing: f32,
}

impl SimpleFilter {
    pub fn new(smoothing: f32) -> SimpleFilter {
        SimpleFilter { smoothing }
    }
}

impl LowPassFilter for SimpleFilter {
    fn apply(&mut self, context: &FilterContext) -> f64 {
        context
            .readings
            .back()
            .map(|r| r.value + self.smoothing as f64 * (context.raw_value - r.value))
            .unwrap_or_else(|| context.raw_value)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{FilterContext, LowPassFilter, SensorData};
    use crate::filter::simple_filter::SimpleFilter;
    use std::collections::VecDeque;

    #[test]
    fn filter_with_no_readings() {
        let mut filter = SimpleFilter::new(0.1);
        let raw_value = 0.5;
        let capacity = 100;
        let readings = VecDeque::with_capacity(capacity);
        let context = FilterContext {
            capacity,
            raw_value,
            readings: &readings,
            timestamp: 4242,
        };

        assert_eq!(filter.apply(&context), raw_value);
    }

    #[test]
    fn filter_with_readings() {
        let capacity = 100;
        let raw_value = 0.5;
        let last_filtered_value = 0.2;
        let smoothing_1 = 0.1;
        let smoothing_2 = 0.2;
        let mut filter_1 = SimpleFilter::new(smoothing_1);
        let mut filter_2 = SimpleFilter::new(smoothing_2);
        let mut readings = VecDeque::with_capacity(capacity);

        let expected_1 = last_filtered_value + smoothing_1 as f64 * (raw_value - last_filtered_value);
        let expected_2 = last_filtered_value + smoothing_2 as f64 * (raw_value - last_filtered_value);

        readings.push_back(SensorData {
            timestamp: 4242,
            value: last_filtered_value,
        });

        let context = FilterContext {
            capacity,
            raw_value,
            readings: &readings,
            timestamp: 4242,
        };

        // TODO: improve using approx_eq instead
        assert_eq!(filter_1.apply(&context), expected_1);
        assert_eq!(filter_2.apply(&context), expected_2);
    }
}

use core::f64;

#[derive(Debug, Copy, Clone)]
pub struct Entry {
    min: f64,
    max: f64,
}

#[derive(Debug, Clone)]
pub struct AutoScale<const N: usize> {
    pub entries: [Entry; N],
    weights: [f64; N],
}

impl<const N: usize> AutoScale<N> {
    pub fn new(weights: [f64; N]) -> AutoScale<N> {
        AutoScale {
            entries: [Entry { min: f64::MAX, max: f64::MIN }; N],
            weights,
        }
    }

    pub fn add(&mut self, values: [f64; N]) {
        // Update entries
        for (entry, value) in self.entries.iter_mut().zip(values.iter()) {
            entry.min = entry.min.min(*value);
            entry.max = entry.max.max(*value);
        }
    }

    pub fn scale(&self, values: [f64; N]) -> f64 {
        let mut score = 0.0;
        for ((entry, value), weight) in self.entries.iter().zip(values.iter()).zip(self.weights.iter()) {
            let range = entry.max - entry.min;
            if range != 0.0 {
                let value = (*value - entry.min) / range;
                score += value * weight;
            }
        }
        score
    }
}
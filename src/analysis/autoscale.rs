use core::f64;

#[derive(Debug, Copy, Clone)]
struct Entry {
    min: f64,
    max: f64,
}

pub struct AutoScale<const N: usize> {
    entries: [Entry; N],
    best_raw: [f64; N],
    weights: [f64; N],
}

impl<const N: usize> AutoScale<N> {
    pub fn new(weights: [f64; N]) -> AutoScale<N> {
        AutoScale {
            entries: [Entry { min: f64::MAX, max: f64::MIN }; N],
            best_raw: [f64::MIN; N],
            weights,
        }
    }

    pub fn add(&mut self, values: [f64; N]) -> Option<[f64; N]> {
        // Update entries
        for (entry, value) in self.entries.iter_mut().zip(values.iter()) {
            entry.min = entry.min.min(*value);
            entry.max = entry.max.max(*value);
        }
        // Rescale best to [0, 1] using min-max scaling
        let mut best_scaled = [0.0; N];
        for i in 0..N {
            best_scaled[i] = ((self.best_raw[i] - self.entries[i].min) / (self.entries[i].max - self.entries[i].min)) * self.weights[i];
        }
        let best_scaled_total = best_scaled.iter().filter(|n| n.is_finite()).sum::<f64>();
        // Rescale values to [0, 1] using min-max scaling
        let mut values_scaled = [0.0; N];
        for i in 0..N {
            values_scaled[i] = ((values[i] - self.entries[i].min) / (self.entries[i].max - self.entries[i].min)) * self.weights[i];
        }
        let values_scaled_total = values_scaled.iter().filter(|n| n.is_finite()).sum::<f64>();
        // Update best if values are better
        if values_scaled_total > best_scaled_total {
            println!("New best: {:?} {:?} {:?} {:?}", values, values_scaled, values_scaled_total, self.entries);
            self.best_raw = values;
            Some(values_scaled)
        } else {
            None
        }
    }
}
use std::fs::File;

use serde::Serialize;

#[derive(Serialize)]
pub struct Trace {
    pub epoch: usize,
    pub fitness: f64,
}

#[derive(Serialize)]
pub struct Traces {
    pub traces: Vec<Trace>,
}

impl Traces {
    pub fn new() -> Self {
        Self { traces: vec![] }
    }

    pub fn add(&mut self, fitness: f64) {
        self.traces.push(Trace {
            fitness,
            epoch: self.traces.len(),
        });
    }

    pub fn get_deviation(&self, window: usize) -> (f64, f64) {
        // Get the deviation of the last window from the mean
        let first = if self.traces.len() >= window {
            self.traces.len() - window
        } else {
            0
        };
        let fitnesses = self
            .traces
            .iter()
            .skip(first)
            .map(|trace| trace.fitness)
            .collect::<Vec<f64>>();
        if fitnesses.is_empty() {
            return (0.0, 0.0);
        }
        let mean = fitnesses.iter().sum::<f64>() / fitnesses.len() as f64;
        let deviation = fitnesses
            .iter()
            .map(|fitness| (fitness - mean).powi(2))
            .sum::<f64>()
            / fitnesses.len() as f64;
        (mean, deviation.sqrt())
    }

    pub fn save(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        let mut writer = serde_json::Serializer::new(&mut file);
        self.serialize(&mut writer).unwrap();
    }
}

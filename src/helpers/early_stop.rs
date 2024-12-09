use crate::helpers::Config;

pub struct EarlyStop {
    pub window: usize,
    pub epochs: usize,
    pub fitness_threshold: f64,
    pub fitness_deviation: f64,
}

impl EarlyStop {
    pub fn new(config: &Config) -> Self {
        Self {
            window: config.early_stop.window,
            epochs: config.early_stop.epochs,
            fitness_threshold: config.early_stop.fitness_threshold,
            fitness_deviation: config.early_stop.fitness_deviation,
        }
    }
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct EarlyStop {
    pub window: usize,
    pub epochs: usize,
    pub fitness_threshold: f64,
    pub fitness_deviation: f64,
}

#[derive(Deserialize)]
pub struct Mutation {
    pub rate: f64,
    pub range: f64,
    pub sections: usize,
}

#[derive(Deserialize)]
pub struct Config {
    pub epochs: usize,
    pub target: String,
    pub mutation: Mutation,
    pub selection_rate: f64,
    pub crossover_rate: f64,
    pub early_stop: EarlyStop,
    pub population_size: usize,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let config = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&config).unwrap()
    }
}

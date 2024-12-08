use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub epochs: usize,
    pub target: String,
    pub mutation_rate: f64,
    pub selection_rate: f64,
    pub crossover_rate: f64,
    pub success_threshold: f64,
    pub population_size: usize,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let config = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&config).unwrap()
    }
}

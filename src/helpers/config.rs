pub struct Config {
    pub epochs: usize,
    pub target: String,
    pub mutation_rate: f64,
    pub selection_rate: f64,
    pub crossover_rate: f64,
    pub success_threshold: f64,
    pub population_size: usize,
}

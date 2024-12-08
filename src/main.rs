mod helpers;

use helpers::{Config, GeneticAlgorithm};

fn main() {
    helpers::init_logger();
    let config = Config {
        epochs: 1000,
        mutation_rate: 0.3, // no mutation
        selection_rate: 0.5,
        crossover_rate: 0.3,
        population_size: 100,
        success_threshold: 0.01,
        target: "image.webp".to_string(),
    };
    let mut ga = GeneticAlgorithm::new(config);
    ga.run();
}

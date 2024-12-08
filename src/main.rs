mod helpers;

use helpers::{Config, GeneticAlgorithm};

fn main() {
    helpers::init_logger();
    let config = Config::new("Config.json");
    let mut ga = GeneticAlgorithm::new(config);
    ga.run();
}

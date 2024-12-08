use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::helpers::image::{load_image, to_image};
use crate::helpers::{Config, Individual};

pub struct GeneticAlgorithm {
    epochs: usize,
    mutation_rate: f64,
    selection_rate: f64,
    crossover_rate: f64,
    success_threshold: f64,
    population_size: usize,
    population: Vec<Individual>,
    target: [Vec<f64>; 3],
    width: u32,
    height: u32,
}

impl GeneticAlgorithm {
    pub fn new(config: Config) -> Self {
        log::info!("initializing genetic algorithm");

        std::fs::create_dir_all(".cache/images/").unwrap();

        log::info!("loading target image");
        let (target, width, height) = load_image(&config.target);

        log::info!("initializing population");
        let population: Vec<Individual> = (0..config.population_size)
            .into_par_iter()
            .map(|i| Individual::new(i, width * height))
            .collect();

        let mut ga = Self {
            epochs: config.epochs,
            mutation_rate: config.mutation_rate,
            selection_rate: config.selection_rate,
            crossover_rate: config.crossover_rate,
            population_size: config.population_size,
            success_threshold: config.success_threshold,
            population,
            target,
            width,
            height,
        };

        log::info!("setting fitness");
        ga.set_fitness();
        ga.save_best(0);

        log::info!("genetic algorithm initialized");
        ga
    }

    pub fn get_parents(&self) -> Vec<usize> {
        let parents_size: usize = ((self.population_size as f64) * self.selection_rate) as usize;
        let parents: Vec<usize> = self
            .population
            .iter()
            .take(parents_size)
            .map(|x| x.id)
            .collect();
        parents
    }

    pub fn crossover(&mut self, parents: Vec<usize>) {
        self.population = (0..self.population_size)
            .into_par_iter()
            .map(|i| {
                let mut rng = StdRng::from_entropy();
                let index1 = rng.gen_range(0..parents.len());
                let index2 = rng.gen_range(0..parents.len());
                let parent1 = &self.population[parents[index1]];
                let parent2 = &self.population[parents[index2]];
                parent1.crossover(parent2, self.crossover_rate, i)
            })
            .collect();
    }

    pub fn mutate(&mut self) {
        self.population.par_iter_mut().for_each(|individual| {
            individual.mutate(self.mutation_rate);
        });
    }

    pub fn set_fitness(&mut self) {
        self.population.iter_mut().for_each(|individual| {
            individual.fitness(&self.target);
        });
        self.population
            .sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
    }

    pub fn save_best(&self, epoch: usize) -> f64 {
        let best = self.population[0].clone();
        let image = to_image(&best.array, self.width, self.height);
        image
            .save(format!(".cache/images/epoch-{}.png", epoch))
            .unwrap();
        image.save(".cache/result.png").unwrap();
        best.fitness
    }

    pub fn evolve(&mut self, epoch: usize) -> f64 {
        let parents = self.get_parents();
        self.crossover(parents);
        self.mutate();
        self.set_fitness();
        self.save_best(epoch)
    }

    pub fn run(&mut self) {
        for epoch in 1..=self.epochs {
            let fitness = self.evolve(epoch);
            println!("epoch: {} -- best fitness: {}", epoch, fitness);
            if fitness < self.success_threshold {
                println!("best fitness reached");
                break;
            }
        }
    }
}

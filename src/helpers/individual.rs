use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Clone)]
pub struct Individual {
    pub id: usize,
    pub fitness: f64,
    pub array: [Vec<f64>; 3],
    pub fitnesses: Vec<f64>,
}

impl Individual {
    pub fn new(id: usize, size: u32) -> Self {
        let mut rng = StdRng::from_entropy();
        let mut array: [Vec<f64>; 3] = [
            vec![0.0; size as usize],
            vec![0.0; size as usize],
            vec![0.0; size as usize],
        ];
        (0..size).for_each(|i| {
            array[0][i as usize] = rng.gen_range(0.0..1.0);
            array[1][i as usize] = rng.gen_range(0.0..1.0);
            array[2][i as usize] = rng.gen_range(0.0..1.0);
        });
        let fitnesses = vec![0.0; size as usize];
        Self {
            id,
            fitness: 0.0,
            array,
            fitnesses,
        }
    }

    pub fn empty(id: usize, size: usize) -> Self {
        Self {
            id,
            fitness: 0.0,
            array: [vec![0.0; size], vec![0.0; size], vec![0.0; size]],
            fitnesses: vec![0.0; size],
        }
    }

    pub fn fitness(&mut self, target: &[Vec<f64>; 3]) {
        self.fitnesses = (0..self.array[0].len())
            .map(|i| {
                (0..3)
                    .map(|j| (self.array[j][i] - target[j][i]).powi(2))
                    .sum::<f64>()
            })
            .collect();
        self.fitness = (self.fitnesses.iter().sum::<f64>() / (self.fitnesses.len() as f64)).sqrt();
    }

    pub fn mutate(&mut self, mutation_rate: f64, mutation_range: f64, mutation_sections: usize) {
        let mut rng = StdRng::from_entropy();
        let grid_size = self.array[0].len() / mutation_sections;

        (0..mutation_sections).for_each(|i| {
            (0..mutation_sections).for_each(|j| {
                let start_x = i;
                let start_y = j;

                let end_x = (i + grid_size).min(mutation_sections);
                let end_y = (j + grid_size).min(mutation_sections);

                let size_x = end_x - start_x;
                let size_y = end_y - start_y;
                let size = size_x * size_y;

                let nb_mutations = (size as f64 * mutation_rate).ceil() as usize;
                let mutations: Vec<(usize, usize, f64)> = (0..nb_mutations)
                    .map(|_| {
                        (
                            rng.gen_range(start_x..end_x),
                            rng.gen_range(start_y..end_y),
                            rng.gen_range(-mutation_range..mutation_range),
                        )
                    })
                    .collect();

                mutations.iter().for_each(|&(i, j, mutation)| {
                    (0..3).for_each(|k| {
                        let index = i * grid_size + j;
                        self.array[k][index] = (self.array[k][index] + mutation).clamp(0.0, 1.0);
                    });
                });
            });
        });
    }

    pub fn crossover(&self, other: &Self, crossover_rate: f64, id: usize) -> Self {
        let nb_pixels = self.array[0].len();
        let mut child = Self::empty(id, nb_pixels);
        let mut rng = StdRng::from_entropy();

        (0..nb_pixels).for_each(|i| {
            let fitness_self = self.fitnesses[i];
            let fitness_other = other.fitnesses[i];
            let total_fitness = fitness_self + fitness_other;
            let prob_other = fitness_other / total_fitness;
            let crossover_other = crossover_rate * prob_other;
            let random_value = rng.gen::<f64>();
            (0..3).for_each(|j| {
                if random_value < crossover_other {
                    child.array[j][i] = self.array[j][i];
                } else {
                    child.array[j][i] = other.array[j][i];
                }
            });
        });

        child.id = id;
        child
    }
}

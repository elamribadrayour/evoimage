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
        self.fitness = self.fitnesses.iter().sum::<f64>() / (self.fitnesses.len() as f64);
    }

    pub fn mutate(&mut self, mutation_rate: f64) {
        let nb_pixels = self.array[0].len();
        let mut rng = StdRng::from_entropy();
        let nb_mutations = (nb_pixels as f64 * mutation_rate).ceil() as usize;
        let mutated_ids: Vec<usize> = (0..nb_mutations)
            .map(|_| rng.gen_range(0..nb_pixels))
            .collect();

        mutated_ids.iter().for_each(|&i| {
            (0..3).for_each(|j| {
                self.array[j][i] = (self.array[j][i] + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0);
            });
        });
    }

    pub fn crossover(&self, other: &Self, _crossover_rate: f64, id: usize) -> Self {
        let nb_pixels = self.array[0].len();
        let mut child = Self::empty(id, nb_pixels);
        (0..nb_pixels).for_each(|i| {
            let fitness_self = self.fitnesses[i];
            let fitness_other = other.fitnesses[i];
            if fitness_self < fitness_other {
                (0..3).for_each(|j| {
                    child.array[j][i] = self.array[j][i];
                });
            } else {
                (0..3).for_each(|j| {
                    child.array[j][i] = other.array[j][i];
                });
            }
        });

        child.id = id;
        child
    }
}

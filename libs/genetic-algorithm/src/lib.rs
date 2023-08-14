use self::selection::*;

use rand::RngCore;

pub struct GeneticAlgorithm {
    sa: SelectionAlgorithm,
    ma: f32,
    ca: f32,
}

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self {
            sa: RouletteWheelSelection,
            ma: 0.1,
            ca: 0.7
        }
    }

    pub fn run<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I> {
        println!("Running genetic algorithm");

        (0..population.len())
            .map(|_| {

                // Perform selection to choose parents
                let parent_a = self.sa.select(rng, population);
                let parent_b = self.sa.select(rng, population);

                // Perform crossover to create child

                // Perform mutation on child
            })
            .collect()
        
    }
}
use crate::selection::*;
use crate::crossover::*;
use crate::chromosomes::*;

mod selection;
mod crossover;
mod chromosomes;

use rand::RngCore;

pub struct GeneticAlgorithm<S, C> {
    sa: S,
    ca: C,
    ma: f32,
}

impl<S, C> GeneticAlgorithm<S, C>
where
    S: SelectionAlgorithm,
    C: CrossoverMethod,
{
    pub fn new(
        selection_alg: S,
        crossover_alg: C,
        mutation_alg: f32,
    ) -> Self {
        
        Self {
            sa: selection_alg,
            ca: crossover_alg,
            ma: 0.1,
        }
    }

    pub fn run<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I> 
    where 
        I: Individual,
    {
        println!("Running genetic algorithm");

        (0..population.len())
            .map(|_| {

                // Perform selection to choose parents
                let parent_a = self.sa.select(rng, population);
                let parent_b = self.sa.select(rng, population);

                // Perform crossover to create child
                let mut child = self.ca.crossover(rng, parent_a.chromosome(), parent_b.chromosome());
                // Perform mutation on child
                todo!()
            })
            .collect()
        
    }
}
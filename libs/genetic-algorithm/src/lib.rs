use crate::selection::*;
use crate::crossover::*;
use crate::chromosomes::*;
use crate::mutation::*;

mod chromosomes;
mod selection;
mod crossover;
mod mutation;

use rand::RngCore;

pub struct GeneticAlgorithm<S, C, M> {
    sa: S,
    ca: C,
    ma: M,
}

impl<S, C> GeneticAlgorithm<S, C, M>
where
    S: SelectionAlgorithm,
    C: CrossoverMethod,
    M: MutationMethod
{
    pub fn new(
        selection_alg: S,
        crossover_alg: C,
        mutation_alg: M,
    ) -> Self {
        
        Self {
            sa: selection_alg,
            ca: crossover_alg,
            ma: mutation_alg,
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
                let mut child_chromosomes = self.ca.crossover(rng, parent_a.chromosome(), parent_b.chromosome());
                
                // Perform mutation on child
                self.ma.mutate(rng, &mut child_chromosomes);

                // Conver the chromosomes into an individual
                todo!()
            })
            .collect()
        
    }
}
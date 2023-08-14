use rand::RngCore;

pub trait SelectionAlgorithm {
    fn select<'a, I>(&self,
                    rng: &mut dyn RngCore,
                    population: &'a[I]) -> &'a I;
    where: 
        I: Individual;
}

pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn run<I>(&self, population: &[I]) -> Vec<I> {
        println!("Running genetic algorithm");

        // TODO: Implement selection, mutation and crossover over the population here.
        
    }
}
pub use crate::{
    selection::*, crossover::*, chromosomes::*, mutation::*; individual::Individual; 
}

mod individual;
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

impl<S, C, M> GeneticAlgorithm<S, C, M>
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
                I::create(child_chromosomes);
            })
            .collect()
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();

        TestIndividual::create(chromosome)
    }

    #[allow(clippy::excessive_precision)] // formatting the numbers differently would make the test less readable
    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::default(),
            UniformCrossover::default(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];

        for _ in 0..10 {
            population = ga.run(&mut rng, &population).0;
        }

        let expected_population = vec![
            individual(&[0.44769490, 2.0648358, 4.3058133]),
            individual(&[1.21268670, 1.5538777, 2.8869110]),
            individual(&[1.06176780, 2.2657390, 4.4287640]),
            individual(&[0.95909685, 2.4618788, 4.0247330]),
        ];

        assert_eq!(population, expected_population);
    }
}
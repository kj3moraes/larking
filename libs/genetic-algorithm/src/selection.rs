
use crate::individual::Individual;
use rand::prelude::SliceRandom;
use rand::{ SeedableRng, RngCore, Rng };
use rand_chacha::ChaCha8Rng;

pub trait SelectionAlgorithm {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a[I]) -> &'a I;
    where: 
        I: Individual;
}

#[derive(Clone, Debug, Default)]
pub struct RouletteWheelSelection;

#[derive(Clone, Debug, Default)]
pub struct KWayTournamentSelection {
    k: usize,
}

impl SelectionAlgorithm for RouletteWheelSelection {
    fn select<'a, I>(&self,
                    rng: &mut dyn RngCore,
                    population: &'a[I]) -> &'a I 
    where:
        I: Individual
    {
        
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
        }
}


impl SelectionAlgorithm for KWayTournamentSelection {
    fn select<'a, I>(&self,
                    rng: &mut dyn RngCore,
                    population: &'a[I]) -> &'a I {
        
        // let mut tournament = Vec::with_capacity(self.k);
        // for _ in 0..self.k {
        //     tournament.push(population.choose(rng).unwrap());
        // }

        // tournament
        //     .iter()
        //     .max_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap())
        //     .unwrap()
        unimplemented!()
    }
}
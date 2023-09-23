use lib_genetic_algorithm::{Chromosome, Individual};
use crate::*;

pub struct AnimalIndividual {
    fitness: f32,
    chromosome: Chromosome,
}

impl Individual for AnimalIndividual {
    fn create(ch: Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome: ch,
        }
    }

    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl AnimalIndividual {

    pub fn from_animal(animal: &Animal) -> Self {
        todo!("Implement AnimalIndividual::from_animal");
    }

    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
        todo!("Implement AnimalIndividual::into_animal");
    }
}
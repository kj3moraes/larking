use rand::{ Rng, RngCore };
use crate::{
    animal::Animal,
    food::Food,
};

pub struct World {
    pub animals: Vec<Animal>,
    pub food: Vec<Food>,
}


impl World {
    pub fn initialize(rng: &mut dyn RngCore) -> Self {

        let animals = (0..40)
                        .map(|_| Animal::random(rng))
                        .collect();
        let food = (0..40)
                        .map(|_| Food::random(rng))
                        .collect();
        Self { animals, food }
    }
}

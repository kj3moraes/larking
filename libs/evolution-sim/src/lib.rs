use lib_neural_network as nn;
use nalgebra as na;
use rand::{ Rng, RngCore };
pub use self::{
    animal::*,
    food::*,
    world::*,
};

pub mod animal;
pub mod food;
pub mod world;

pub struct Simulation {
    pub world: World,
    pub age: usize
}


impl Simulation {
    pub fn initialize(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::initialize(rng),
            age: 0
        }
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.age += 1;
        self.process_collisions(rng);
        self.process_movement();
        // self.generate_food(rng);
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) -> usize{
        
        // Count the number of collisions that take place 
        let mut collisions: usize = 0;

        // Remove animals that are on top of food
        for animal in &mut self.world.animals {
            for food in &mut self.world.food {
                if na::distance(&animal.position, &food.position) < 0.01 {
                    food.position = rng.gen();
                    collisions += 1;
                }
            }
        }

        collisions
    }

    fn process_movement(&mut self) {
        for animal in &mut self.world.animals {
            animal.budge();

            // Flip them around if they are out of bounds 
            if animal.position.x < 0.0 || animal.position.x > 1.0 {
                animal.rotation *= na::Rotation2::new(std::f32::consts::PI / 2.0);
            }
            if animal.position.y < 0.0 || animal.position.y > 1.0 {
                animal.rotation *= na::Rotation2::new(std::f32::consts::PI / 2.0);
            }
        }
    }

    fn generate_food(&mut self, rng: &mut dyn RngCore) {
       todo!();
    }
}

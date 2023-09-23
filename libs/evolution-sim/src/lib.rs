use nalgebra as na;
use rand::{ Rng, RngCore };
use lib_neural_network as nn;
use lib_genetic_algorithm as ga;
pub use self::{
    animal::*,
    food::*,
    world::*,
    animal_individual::*,
};

pub mod eye;
pub mod animal;
pub mod food;
pub mod world;
mod animal_individual;

use std::f32::consts::FRAC_PI_2;
use lib_genetic_algorithm::GeneticAlgorithm;

// The minimum speed a bird can have
const SPEED_MIN: f32 = 0.001;

// The maximum speed a bird can have
const SPEED_MAX: f32 = 0.005;

// Speed acceleration; determines how much the brain can affect bird's
// speed during one step.
const SPEED_ACCEL: f32 = 0.2;

// Rotation acceleration; determines how much the brain can affect bird's
// rotation during one step.    
const ROTATION_ACCEL: f32 = FRAC_PI_2 / 4.0;

// After how many steps should the world evolve
const GENERATION_LENGTH: usize = 2500;

pub struct Simulation {
    pub world: World,
    pub age: usize,
    ga: GeneticAlgorithm<ga::RouletteWheelSelection, ga::UniformCrossover, ga::GaussianMutation>,
}


impl Simulation {
    pub fn initialize(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::initialize(rng),
            age: 0,
            ga: GeneticAlgorithm::new(
                ga::RouletteWheelSelection::default(),
                ga::UniformCrossover::default(),
                ga::GaussianMutation::new(0.01, 0.3),
            ),
        }
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.age += 1;
        self.process_collisions(rng);
        if self.age % 5 == 0 {
            self.process_brains();
        }
        self.process_movement();

        if self.age % GENERATION_LENGTH == 0 {
            self.evolve(rng)
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) {

        // Step 1: Get the current population
        let curr_population =
            self.world.animals
                .iter()
                .map(AnimalIndividual::from_animal)
                .collect();

        // Step 2: Evolve the current population
        let evolved_population = self.ga.run(
            rng,
            &curr_population
        );

        // Step 3: Figure our how to brind them back
        self.world.animals =
            evolved_population
                .into_iter()
                .map(|individual| individual.into_animal(rng))
                .collect();

        // Step 4: Scatter the food (for UI sake)
        for food in &mut self.world.food {
            food.position = rng.gen();
        }
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

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision = animal.eye.process_vision(
                animal.position,
                animal.rotation,
                &self.world.food
            );

            let response = animal.brain.propogate(vision);

            let speed_accel = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation_accel = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);
        
            animal.speed = (animal.speed + speed_accel).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation *= na::Rotation2::new(
                animal.rotation.angle() + rotation_accel
            );
        }
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

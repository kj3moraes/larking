use nalgebra as na;
use rand::{ Rng, RngCore };
use lib_neural_network as nn;
pub use self::{
    animal::*,
    food::*,
    world::*,
};

pub mod eye;
pub mod animal;
pub mod food;
pub mod world;

use std::f32::consts::FRAC_PI_2;

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
        if self.age % 5 == 0 {
            self.process_brains();
        }
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

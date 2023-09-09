use nalgebra as na;
use rand::{ Rng, RngCore };

pub struct Simulation {
    pub world: World,
    pub age: usize
}

pub struct World {
    pub animals: Vec<Animal>,
    pub food: Vec<Food>,
}

#[derive(Clone, Debug)]
pub struct Animal {
    pub position: na::Point2<f32>,
    pub rotation: na::Rotation2<f32>,
    pub speed: f32,
}

#[derive(Clone, Debug)]
pub struct Food {
    pub position: na::Point2<f32>,
}


impl Simulation {
    pub fn initialize(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::initialize(rng),
            age: 0
        }
    }
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


impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: na::Point2::new(rng.gen(), rng.gen()),
            rotation: na::Rotation2::new(rng.gen()),
            speed: 0.002
        }
    }
}


impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: na::Point2::new(rng.gen(), rng.gen()),
        }
    }
}
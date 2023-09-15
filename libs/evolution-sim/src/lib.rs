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


impl World {
    pub fn initialize(rng: &mut dyn RngCore) -> Self {

        let animals = (0..40)
                        .map(|_| Animal::random(rng))
                        .collect();
        let food = (0..60)
                        .map(|_| Food::random(rng))
                        .collect();
        Self { animals, food }
    }
}


impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002
        }
    }

    pub fn budge(&mut self) {
        self.position += self.rotation * na::Vector2::new(0.0, self.speed);
    }
}


impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: na::Point2::new(rng.gen(), rng.gen()),
        }
    }
}
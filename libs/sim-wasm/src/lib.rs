use wasm_bindgen::prelude::*;
use lib_evolution as sim;
use rand::prelude::*;

#[wasm_bindgen]
pub fn tell_em() -> String {
    "\"The Reverend Mother must combine the seductive wiles of a courtesan with the untouchable majesty of a virgin goddess".to_string()
}

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    animals: Vec<Animal>,
    food: Vec<Food>,
}


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Animal {
    x: f32,
    y: f32,
    rotation: f32
}


#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut sim = sim::Simulation::initialize(&mut trng);
        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world)
    }
}


impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        Self {
            animals: world.animals
                            .iter()
                            .map(|a| a.into())
                            .collect(),
            food: world.food
                        .iter()
                        .map(|f| f.into())
                        .collect(),
        }
    }
}


impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position.x,
            y: animal.position.y,
        }
    }
}
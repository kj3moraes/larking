use wasm_bindgen::prelude::*;
use lib_evolution as sim;
use rand::prelude::*;
use serde::{ Serialize, Deserialize };
use gloo_utils::format::JsValueSerdeExt;

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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct World {
    animals: Vec<Animal>,
    food: Vec<Food>,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32
}

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Food {
    pub x: f32,
    pub y: f32
}


#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut sim = sim::Simulation::initialize(&mut rng);
        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(&self.sim.world);
        JsValue::from_serde(&world).unwrap()
    }

    pub fn animals_present(&self) -> JsValue {
        let world = World::from(&self.sim.world);
        let animals = world.animals.clone();
        JsValue::from_serde(&animals).unwrap()
    }
}


impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        Self {
            animals: world.animals
                            .iter()
                            .map(Animal::from)
                            .collect(),
            food: world.food
                        .iter()
                        .map(Food::from)
                        .collect(),
        }
    }
}


impl From<&sim::Animal> for Animal {
    fn from(animal: &sim::Animal) -> Self {
        Self {
            x: animal.position.x,
            y: animal.position.y,
            rotation: animal.rotation.angle()
        }
    }
}


impl From<&sim::Food> for Food {
    fn from(food: &sim::Food) -> Self {
        Self {
            x: food.position.x,
            y: food.position.y,
        }
    }
}
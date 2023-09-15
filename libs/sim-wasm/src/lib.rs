use rand::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use serde::{ Serialize, Deserialize };
use gloo_utils::format::JsValueSerdeExt;

use lib_evolution as sim;

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
        let sim = sim::Simulation::initialize(&mut rng);
        Self { rng, sim }
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(&self.sim.world);
        JsValue::from_serde(&world).unwrap()
    }
}

#[wasm_bindgen]
impl World {

    pub fn food_present(&self) -> JsValue {
        JsValue::from_serde(&self.food).unwrap()
    }

    pub fn animals_present(&self) -> JsValue {
        JsValue::from_serde(&self.animals).unwrap()
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
use wasm_bindgen::prelude::*;
use lib_evo_sim as sim;
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
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut sim = sim::Simulation::initialize(&mut trng);
        Self { rng, sim }
    }
}


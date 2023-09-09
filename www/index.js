import * as sim from "lib-sim-wasm";

const simulation = new sim.Simulation();
const world = simulation.world();
const animals = simulation.animals_present();
console.log(world);
console.log(animals)
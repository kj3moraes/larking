use crate::*;
use rand::{ Rng, RngCore };
use crate::activation::{ReLU, Sigmoid};

// Each neuron in the FFNN is defined by its weights and bias.
pub struct Neuron {
    pub(crate) weights: Vec<f32>,
    pub(crate) bias: f32,
}


impl Neuron {

    pub fn new(bias: f32, weights: Vec<f32>) -> Self {
        Self { bias, weights }
    }

    pub fn random(rng: &mut dyn RngCore, input_weights: usize) -> Self {

        let bias = rng.gen_range(-1.0..1.0);
        let weights = (0..input_weights)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();

        Self::new(bias, weights)
    }

    pub fn from_weights(output_neurons: usize, weights: &mut dyn Iterator<Item= f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..output_neurons)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self::new(bias, weights)
    }

    pub fn propogate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let sum:f32 = inputs
            .iter()
            .zip(&self.weights)
            .map(|(weight, input)| weight * input)
            .sum();

        ReLU(sum + self.bias)
    }
}
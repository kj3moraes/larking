use crate::*;

use crate::activation::{ReLU, Sigmoid};

// Each neuron in the FFNN is defined by its weights and bias.
pub struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}


impl Neuron {

    pub fn new(input_weights: usize) -> Self {
        let mut rng = rand::thread_rng();

        let bias = rng.gen_range(-1.0..1.0);
        let weights = (0..input_weights)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();

        Self { weights, bias }
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
use rand::{ Rng, RngCore };
use crate::{neurons::Neuron, layer::Layer};

pub mod activation;
pub mod layer;
mod neurons;
// The topology of the neural network is defined by the number of neurons in each layer.
// The user must provide a vector of Topology structs to the NeuralNetwork constructor.
// We assume that every layer is fully connected to the previous layer.
pub struct Topology {
    neurons: usize,
}

pub struct NeuralNetwork {
    layers: Vec<Layer>,
}

impl NeuralNetwork {

    pub(crate) fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn from_weights(layers: &[Topology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self::new(layers)
    }

    pub fn random(rng: &mut dyn RngCore, topology: &[Topology]) -> Self {
        assert!(!topology.is_empty());

        let layers = topology
                        .windows(2)
                        .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
                        .collect();

        Self::new(layers)
    }

    pub fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propogate(inputs))
    }
}
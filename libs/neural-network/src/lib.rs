use rand::Rng;

use crate::neurons::Neuron;

pub mod activation;
mod neurons;

struct Layer {
    neurons: Vec<Neuron>,
}


impl Layer {

    fn new(input_neurons:usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
                    .map(|_| Neuron::new(input_neurons))
                    .collect();
        Self { neurons }
    }

    fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propogate(&inputs))
            .collect()
    }
}


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

    pub fn new(topology: &[Topology]) -> Self {
        assert!(topology.len() >= 2);

        let layers = topology
            .windows(2)
            .map(|layers| {
                Layer::new(layers[0].neurons, layers[1].neurons)
            })
            .collect();
        
        Self { layers }
    }

    pub fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propogate(inputs))
    }
}
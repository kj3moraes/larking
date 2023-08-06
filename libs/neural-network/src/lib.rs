
// ACTIVATION FUNCTIONS

pub fn Sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

pub fn ReLU(x: f32) -> f32 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

// NEURAL NETWORK

struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}


impl Neuron {
    fn propogate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len())

        let mut sum = inputs
            .iter()
            .zip(&self.weights)
            .map(|(weight, input)| weight * input)
            .sum();

        ReLU(sum + self.bias)
    }
}


struct Layer {
    neurons: Vec<Neuron>,
}


impl Layer {
    fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propogate(&inputs))
            .collect()
    }
}


pub struct NeuralNetwork {
    layers: Vec<Layer>,
}


impl NeuralNetwork {
    pub fn propogate(&self, inputs: Vec<f32>) -> Vec<f32> {
        
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propogate(inputs))
    }
}
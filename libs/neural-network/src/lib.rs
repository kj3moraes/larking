use rand::Rng;


// ACTIVATION FUNCTIONS
pub fn Sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

pub fn ReLU(x: f32) -> f32 {
    if x > 0.0 {
        x.min(1.0)
    } else {
        0.0
    }
}

// NEURAL NETWORK

// Each neuron in the FFNN is defined by its weights and bias.
struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}


impl Neuron {

    fn new(input_weights: usize) -> Self {
        let mut rng = rand::thread_rng();

        let bias = rng.gen_range(-1.0..1.0);
        let weights = (0..input_weights)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();

        Self { weights, bias }
    }

    fn propogate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let sum:f32 = inputs
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

    pub fn new(topology: Vec<Topology>) -> Self {
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
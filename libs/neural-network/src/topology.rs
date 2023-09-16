// The topology of the neural network is defined by the number of neurons in each layer.
// The user must provide a vector of Topology structs to the NeuralNetwork constructor.
// We assume that every layer is fully connected to the previous layer.
#[derive(Clone, Copy, Debug)]
pub struct Topology {
    pub neurons: usize,
}
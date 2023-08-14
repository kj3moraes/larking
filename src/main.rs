
mod neural_network;

fn main() {
    let topology = vec![
        Topology { neurons: 2 },
        Topology { neurons: 3 },
        Topology { neurons: 1 },
    ];
    let nn = NeuralNetwork::new(&topology);

    println!("Hello, world!");
}

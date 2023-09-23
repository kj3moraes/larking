use crate::*;
use lib_neural_network as nn;
use crate::eye::Eye;
use lib_genetic_algorithm as ga;

#[derive(Clone, Debug)]
pub struct Brain {
    pub(crate) nn: nn::NeuralNetwork,
}

impl Brain {
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: nn::NeuralNetwork::random(
                rng,
                &Self::topology(eye)
            ),
        }
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, eye: &Eye) -> Self {
        Self {
            nn: nn::NeuralNetwork::from_weights(
                &Self::topology(&eye),
                chromosome,
            ),
        }
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.get_weights().collect()
    }

    fn topology(eye: &Eye) -> [nn::topology::Topology; 3]{
        [
            nn::topology::Topology {
                neurons: eye.cells(),
            },
            nn::topology::Topology {
                neurons: 2 * eye.cells(),
            },
            nn::topology::Topology {
                neurons: 2,
            },
        ]
    }
}
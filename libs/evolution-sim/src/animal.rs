use nalgebra as na;
use rand::{ Rng, RngCore };
use crate::*;

#[derive(Clone, Debug)]
pub struct Animal {
    pub position: na::Point2<f32>,
    pub rotation: na::Rotation2<f32>,
    pub speed: f32,

    pub(crate) eye: eye::Eye,
    pub(crate) brain: nn::NeuralNetwork,
}


impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = eye::Eye::default();
        let brain = nn::NeuralNetwork::random(
            rng,
            &[
                nn::topology::Topology { neurons: eye.cells() },
                nn::topology::Topology { neurons: 2 * eye.cells() },
                nn::topology::Topology { neurons: 2 },
            ]
        );

        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain
        }
    }

    pub fn budge(&mut self) {
        self.position += self.rotation * na::Vector2::new(0.0, self.speed);
    }
}


use nalgebra as na;
use rand::{ Rng, RngCore };
use crate::eye::*;
use crate

#[derive(Clone, Debug)]
pub struct Animal {
    pub position: na::Point2<f32>,
    pub rotation: na::Rotation2<f32>,
    pub speed: f32,

    crate eye: Eye,
    crate brain: nn::Network,
}


impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        // let brain = nn::Network::random(
        //     rng,
        //     &[
        //         eye.cells(),
        //         3,
        //         2
        //     ]
        // );

        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002
        }
    }

    pub fn budge(&mut self) {
        self.position += self.rotation * na::Vector2::new(0.0, self.speed);
    }
}


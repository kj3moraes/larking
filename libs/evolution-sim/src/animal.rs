use nalgebra as na;
use rand::{ Rng, RngCore };
use crate::*;
use crate::brain::Brain;
use crate::eye::Eye;

#[derive(Clone, Debug)]
pub struct Animal {
    pub position: na::Point2<f32>,
    pub rotation: na::Rotation2<f32>,
    pub speed: f32,

    // number of foods eaten by this animal
    pub(crate) consumed: usize,

    // the features that give the animal its functionality
    pub(crate) eye: eye::Eye,
    pub(crate) brain: brain::Brain,
}


impl Animal {

    pub fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            consumed: 0,
            eye,
            brain,
        }
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = eye::Eye::default();
        let brain = Brain::random(rng, &eye);

        Self::new(eye, brain, rng)
    }

    pub fn budge(&mut self) {
        self.position += self.rotation * na::Vector2::new(0.0, self.speed);
    }

    pub(crate) fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = eye::Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
        self.brain.as_chromosome()
    }

}


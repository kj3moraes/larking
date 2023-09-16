use nalgebra as na;
use rand::{ Rng, RngCore };

#[derive(Clone, Debug)]
pub struct Food {
    pub position: na::Point2<f32>,
}


impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: na::Point2::new(rng.gen(), rng.gen()),
        }
    }
}

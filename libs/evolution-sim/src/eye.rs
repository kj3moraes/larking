use std::f32::consts::*;
use nalgebra as na;
use crate::food::Food;

const DEFAULT_FOV_RANGE: f32 = 0.1;
const DEFAULT_FOV_ANGLE: f32 = PI / 4.0 // (45 degrees);
const DEFAILT_CELLS: usize = 9;


#[derive(Clone, Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize
}


impl Eye {
    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self { fov_range, fov_angle, cells }
    }

    pub fn cells (&self) -> usize {
        self.cells
    }

    pub fn process_vision(&mut self,
                            position: na::Point2<f32>,
                            rotation: na::Rotation2<f32>,
                            foods: &[Food],
    ) -> Vec<f32> {
        todo!()
    
    }
}


impl Default for Eye {
    fn default() -> Self {
        Self::new(DEFAULT_FOV_RANGE, DEFAULT_FOV_ANGLE, DEFAILT_CELLS)
    }
}
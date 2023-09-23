use std::f32::consts::*;
use nalgebra as na;
use crate::food::Food;

const DEFAULT_FOV_RANGE: f32 = 0.5;
const DEFAULT_FOV_ANGLE: f32 = FRAC_PI_4 * 1.5; 
const DEFAULT_CELLS: usize = 4;


#[derive(Clone, Debug)]
pub struct Eye {
    pub(crate) fov_range: f32,
    pub(crate) fov_angle: f32,
    pub(crate) cells: usize
}


fn softmax(vec: &[f32]) -> Vec<f32> {
    let mut sum: f32 = vec.iter().sum();
    let mut result = vec![0.0; vec.len()];
    for i in 0..vec.len() {
        result[i] = vec[i] / sum;
    }
    result
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
        let mut cells = vec![0.0; self.cells];
        
        for food in foods {
            let vec = food.position - position;
            let distance = vec.norm();
            if distance > self.fov_range {
                continue;
            }
            
            // Calculate the angle with respect to the x-axis
            let angle = na::Rotation2::rotation_between(
                                &na::Vector2::y(),
                                &vec
                            ).angle();
            // Subtract the birds own rotation
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            // Makes angle *relative* to our birdie's field of view - that is:
            // transforms it from <-FOV_ANGLE/2,+FOV_ANGLE/2> to <0,FOV_ANGLE>.
            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            // Energy needed to expend to get the food. 
            let energy = (self.fov_range - distance) / self.fov_range;

            cells[cell] += energy;
    
        }   
        let norm_cells = softmax(&cells);
        norm_cells
    }
}


impl Default for Eye {
    fn default() -> Self {
        Self::new(DEFAULT_FOV_RANGE, DEFAULT_FOV_ANGLE, DEFAULT_CELLS)
    }
}
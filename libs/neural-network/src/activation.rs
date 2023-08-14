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
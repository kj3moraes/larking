// ACTIVATION FUNCTIONS

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

pub fn relu(x: f32) -> f32 {
    if x > 0.0 {
        x.min(1.0)
    } else {
        0.0
    }
}

pub fn softmax(vec: &[f32]) -> Vec<f32> {
    let sum: f32 = vec.iter().sum();
    let mut result = vec![0.0; vec.len()];
    for i in 0..vec.len() {
        result[i] = vec[i] / sum;
    }
    result
}
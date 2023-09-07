use crate::*;

pub mod abilities;

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>
}

impl Chromosome {
    pub fn new(genes: Vec<f32>) -> Self {
        Self { genes }
    }

    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item=&f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut f32> {
        self.genes.iter_mut()
    }
}

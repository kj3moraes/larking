use std::iter::{FromIterator, IntoIterator};
use std::ops::{Index, IndexMut};

use self::chromosome::Chromosome;

impl Index for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}


impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item=f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect()
        }
    }
}


impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

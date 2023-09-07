
pub trait MutationMethod {
    fn mutate(
        &self,
        rng: &mut dyn RngCore,
        chromosome: &mut Chromosome,
    );
}

#[derive(Clone, Debug)]
pub struct GaussianMutation {
    chance: f32,

    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {

    pub fn mutate(&self, rng: &mut dyn RngCore, chromosome: &mut Chromosome) {
        for gene in chromosome.iter_mut() {
            let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            if rng.gen_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }

}
use crate::chromosomes::Chromosome;
use rand::Rng;
use rand::RngCore;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}

#[derive(Clone, Debug, Default)]
pub struct UniformCrossover;


impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome {
        assert!(parent_a.len() > 0);
        assert!(parent_a.len() == parent_b.len());

        let mut child = Vec::new();
        let gene_count = parent_a.len();

        for gene_idx in 0..gene_count {

            // Flip a coin to decide which parent to take the gene from
            let gene = if rng.gen_bool(0.5) {
                parent_a[gene_idx]
            } else {
                parent_b[gene_idx]
            };
            
            child.push(gene);
        }

        child.into_iter().collect()
    }
}
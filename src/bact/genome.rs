pub struct Genome {
    pub genes: Vec<u8>,
}

impl Genome {
    pub fn mutate(&mut self, rng: ThreadRng) {
        let mut_rate = self.genes[0];
        let mut_count = self.genes[1];
        if rng.gen_range(0..100) < mut_rate {
            for _ in 0..mut_count {
                self.genes[rng.gen_range(0..self.genes.len())] = rng.gen_rage(0..=255);
            }
        }
    }
}

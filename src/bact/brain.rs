use crate::bact::genome::Genome;

const NODE_COUNT: usize = 64;

pub struct Brain {
    connections: Vec<(usize, usize, f64)>,
    nodes: [f64; NODE_COUNT],
}

impl Brain {
    pub fn from_genome(genome: &Genome) -> Self {
        let mut connections: Vec<(usize, usize, f64)> = Vec::new();
        let mut i = 0;
        while i < genome.genes.len() {
            connections.push((
                    genome.genes[i] as usize,
                    genome.genes[i+1] as usize,
                    (genome.genes[i+2] as f64 - 128.0) / 32.0
                    ));
            i += 3;
        }

        Self {
            connections,
            nodes: [0.0; NODE_COUNT],
        }
    }

    pub fn step(&mut self) {
        let mut deltas = [0.0f64; NODE_COUNT];

        for (src, tgt, weight) in self.connections.iter() {
            deltas[*tgt] += self.nodes[*src] * *weight;
        }

        for i in 0..NODE_COUNT {
            self.nodes[i] = 0.5 * self.nodes[i] + 0.5 * deltas[i].tanh();
        }
    }
}

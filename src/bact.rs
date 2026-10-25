mod genome;
mod brain;

use crate::pos::Pos;
use genome::Genome;
use brain::Brain;

type Energy = f64;

pub struct Bact {
    pub pos: Pos,
    pub energy: Energy,
    pub genome: Genome,
    pub brain: Brain,
}

impl Bact {
    pub fn step(&mut self) {
        self.pos.y += 1.0;
    }
}

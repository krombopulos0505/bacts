# Bacts

## Description

Bacts is the a-life simulation using GRN-like brain behavior.

## Demo

## Bacts Brain Architecture

Bact's brain is an array of count of "proteins" in each node of the brain. 
The agent's control architecture is implemented as a Gene Regulatory Network 
(GRN), modeled as a directed weighted graph. Each node represents the current 
concentration (expression level) of a specific protein, while the edges define 
the weights of regulatory interactions. The network receives input stimuli from 
sensory organs. The final behavioral output is determined via a Winner-Take-All 
mechanism, selecting the node with the highest protein concentration at the 
current simulation timestep.

## How to install and run

```bash
git clone https://github.com/krombopulos0505/bacts.git
cd bacts
cargo build --release
cargo run
```

## License
MIT - [LICENSE](LICENSE)

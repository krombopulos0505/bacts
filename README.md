# Bacts

## Description

Bacts is the a-life simulation using GRN-like brain behavior.
World is a discrete field of cells.

## Demo

## How to install and run

```bash
git clone https://github.com/krombopulos0505/bacts.git
cd bacts
cargo build --release
cargo run
```

## Bacts Brain Architecture

The agent's control architecture is implemented as a Gene Regulatory Network 
(GRN), modeled as a directed weighted graph. Each node represents the current 
concentration (expression level) of a specific protein, while the edges define 
the weights of regulatory interactions. The network receives input stimuli from 
sensory organs. The final behavioral output is determined via a Winner-Take-All 
mechanism, selecting the node with the highest protein concentration at the 
current simulation timestep.

## Bacts Architecture

Bact is a unit of agent.
 - Bact's genome structure:

    ```
    genome:
        genes 0..4: organs
        genes 5..256: connections of nodes
    ```
    
    - Organs:
        1. Eye - senses a color
        2. Chlorophile - produces energy from photons
        3. Agility - movement
        4. Jaw - power of jaw
        5. Stomach - produces energy from organics

        Each organ affects metabolism

    - Connections:
        a connection contains of source, target, and weight

## License
MIT - [LICENSE](LICENSE)

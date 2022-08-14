# Lattice-type

## Description
calculate the ground-state energy (typically, Lennard-Jones potential or exp-6 potential) to find the phase type of a solid

## Input
+ the program receive the input parameters from a json file call "input.json" in the same dictory 
```python
# input.json
{
  "name":"Cu",  # name of the solid
  "epochs":1000, # max iter epochs of the calculation
  "potential_type":"LJ", # the energy among atoms
  "edge":8, # we only consider the energy between atoms that close enough (inside the circle which diameter = edge)
  "lattice_type":"fcc", # lattice type 
  "LJ_params":{ # input parameters when using Lennard-Jones potential
    "sigma": 1,
    "epsilon": 0.05
  },
  "exp6_params":{  # input parameters when using exp-6 potential
    "alpha":1,
    "A":1,
    "C":0.00002
  },
  "step":0.02, # the change step of lattice parameter a each step
  "periodic":7, # the copy of supercell in each axis
  "init_lattice_param": 0.2 # the lattice parameter a used in the begining of the calculation
}
```

## Usage
+ Run the code, this will list the energy and a of each epoch, and finally give the lowest energy of the calculation.
```bash
cargo run
```
+ Release a binary program

```bash
cargo build --release
```

## Reference
+ <a href="https://en.wikipedia.org/wiki/Lennard-Jones_potential">Lennard-Jones potential</a>
+ <a href="https://en.wikipedia.org/wiki/Buckingham_potential">exp-6 potential</a>
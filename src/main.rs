#![allow(non_snake_case)]

use lattice_type::config::Config;
use lattice_type::lattice;
use lattice_type::potentials::{Exp6, Potential, LJ};

fn combi2(n: usize) -> Vec<Vec<usize>> {
    let mut ret = vec![];
    for i in 0..n {
        for j in i + 1..n {
            ret.push(vec![i, j]);
        }
    }
    ret
}

fn main() {
    // read input
    let config = Config::readCfg("./input.json");

    // decide the energy type
    let potential: Box<dyn Potential>;
    if config.potential_type == "LJ" {
        potential = Box::new(LJ::new(config.LJ_params.sigma, config.LJ_params.epsilon))
            as Box<dyn Potential>;
    } else if config.potential_type == "exp6" || config.potential_type == "exp-6" {
        potential = Box::new(Exp6::new(
            config.exp6_params.alpha,
            config.exp6_params.A,
            config.exp6_params.C,
        ));
    } else {
        unreachable!()
    }

    let mut energy;
    let mut energy_before = 0f64;
    let mut atoms = lattice::periodic(
        config.init_lattice_param,
        config.periodic,
        &config.lattice_type,
    );
    let mut a = config.init_lattice_param;

    let combi = combi2(atoms.shape()[0]);

    println!("calculation begin, type : {:?}", config.lattice_type);

    for epoch in 0..config.epochs {
        energy = 0f64;
        for atomSet in &combi {
            let mut r = 0f64;
            for axis in 0..3 {
                r += (atoms.row(atomSet[0])[axis] - atoms.row(atomSet[1])[axis])
                    .abs()
                    .powf(2.);
            }
            r = r.sqrt();
            // only consider the atom close enough
            if r < config.edge && r >= 1e-6 {
                energy += potential.getEnergy(r);
            }
        }
        println!("energy is {:?} , a is {:?} in epoch {:?}", energy, a, epoch);

        // update a
        if epoch == 0 {
            energy_before = energy + 1.;
        }
        if energy_before < energy {
            println!("reach the lowest energy");
            break;
        }
        a += config.step;
        atoms = lattice::periodic(a, config.periodic, &config.lattice_type);
        energy_before = energy;
    }
    println!(
        "iter end, the final energy is {:?} per atom",
        energy_before / atoms.shape()[0] as f64
    );
}

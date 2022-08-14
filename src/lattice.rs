use ndarray::prelude::*;
use ndarray::{Array, Ix2};

pub fn fcc(posi: (f64, f64, f64), a: f64) -> Array<f64, Ix2> {
    array![
        [posi.0, posi.1, posi.2],
        [0.5 * a + posi.0, 0.5 * a + posi.1, posi.2],
        [posi.0, 0.5 * a + posi.1, 0.5 * a + posi.2],
        [0.5 * a + posi.0, posi.1, 0.5 * a + posi.2],
    ]
}

pub fn hcp(posi: (f64, f64, f64), a: f64) -> Array<f64, Ix2> {
    array![
        [posi.0, posi.1, posi.2],
        [a + posi.0, posi.1, posi.2],
        [0.5 * a + posi.0, 0.866 * a + posi.1, posi.2],
        [posi.0, posi.1, 1.633*a + posi.2],
    ]
}

pub fn periodic(a: f64, periodic: u32, lattice_type: &str) -> Array<f64, Ix2> {
    let mut atoms: Array2<f64> = fcc((0., 0., 0.), a);
    if lattice_type == "hcp" {
        atoms = hcp((0., 0., 0.), a);
    }
    for i in 0..periodic {
        for j in 0..periodic {
            for k in 0..periodic {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                };
                if lattice_type == "hcp" {
                    let newCell = hcp((i as f64 * a, j as f64 * a, k as f64 * a), a);
                    for atom in newCell.rows() {
                        atoms.push(Axis(0), atom).unwrap();
                    }
                } else if lattice_type == "fcc" {
                    let newCell = fcc((i as f64 * a, j as f64 * a, k as f64 * a), a);
                    for atom in newCell.rows() {
                        atoms.push(Axis(0), atom).unwrap();
                    }
                }
            }
        }
    }

    atoms
}

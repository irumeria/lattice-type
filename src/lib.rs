#![allow(non_snake_case)]
pub mod config;
pub mod potentials;
pub mod lattice;

pub fn combi2(n: usize) -> Vec<Vec<usize>> {
    let mut ret = vec![];
    for i in 0..n {
        for j in i + 1..n {
            ret.push(vec![i, j]);
        }
    }
    ret
}
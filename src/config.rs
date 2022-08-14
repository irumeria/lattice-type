#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct LJ_params{
    pub sigma:f64,
    pub epsilon:f64
}

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Exp6_params{
    pub alpha:f64,
    pub A:f64,
    pub C:f64
}

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Config {
    pub name: String,
    pub epochs: u32,
    pub potential_type:String,
    pub LJ_params:LJ_params,
    pub exp6_params:Exp6_params,
    pub init_lattice_param: f64,
    pub periodic:u32,
    pub edge:f64,
    pub step:f64,
    pub lattice_type:String
}

impl Config {
    pub fn readCfg(path: &str) -> Config {
        let config: Config = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        config
    }

    pub fn readCfg2Str(path: &str) -> String {
        let config: Config = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();

        serde_json::to_string(&config).unwrap()
    }
}

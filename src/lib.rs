#![feature(proc_macro_hygiene, decl_macro)]

pub mod docker;
pub mod router;

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    languages: Vec<String>,
}

impl Config {
    pub fn from_file(path: &str) -> Config {
        let toml_str = fs::read_to_string(path).unwrap();
        toml::from_str(&toml_str).unwrap()
    }
}

pub mod us_metar_components;
pub mod world_metar;
pub mod parser;
pub mod taf_only_groups;
use crate::parser::*;
use std::env;

fn main() {
    let raw: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < raw.len() {
        metar_or_taf(&raw.get(i).unwrap()[..]);
        i += 1;
    }
}





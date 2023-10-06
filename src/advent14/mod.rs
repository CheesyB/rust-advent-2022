mod domain;

use std::fs;

use crate::advent14::domain::*;
use crate::helper;

pub fn advent14() -> String {
    let content = helper::read_puzzle_input("./src/advent14/rocks_test.txt");
    let mut map = Map::new(&content);
    simulate(&mut map, &START_POSITION);

    fs::write("src/advent14/render.txt", map.to_string()).expect("things to just work");

    "not implemented".into()
}
pub fn advent14_2() -> String {
    let content = helper::read_puzzle_input("./src/advent14/rocks.txt");
    "not implemented".into()
}

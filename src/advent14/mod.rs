mod domain;

use std::fs;

use crate::advent14::domain::*;
use crate::helper;

pub fn advent14() -> String {
    let content = helper::read_puzzle_input("./src/advent14/rocks.txt");
    let mut map = Map::new(&content);
    fs::write("src/advent14/render_raw.txt", map.to_string()).expect("things to just work");
    let result = simulate_void(&mut map, &START_POSITION);
    fs::write("src/advent14/render_result.txt", map.to_string()).expect("things to just work");
    result.to_string()
}
pub fn advent14_2() -> String {
    let content = helper::read_puzzle_input("./src/advent14/rocks.txt");
    let mut map = Map::new(&content);
    fs::write("src/advent14/render_raw.txt", map.to_string()).expect("things to just work");
    let result = simulate_unitl_full(&mut map, &START_POSITION);
    fs::write("src/advent14/render_result.txt", map.to_string()).expect("things to just work");
    result.to_string()
}

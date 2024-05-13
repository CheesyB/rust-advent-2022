mod algo;
mod domain;

use crate::advent17::algo::*;

use crate::helper::read_puzzle_input;

pub fn advent17() -> String {
    let content = read_puzzle_input("./src/advent17/rocks.txt");
    simulate(&content, 2022).to_string()
}

pub fn advent17_2() -> String {
    let content = read_puzzle_input("./src/advent17/rocks.txt");
    simulate2(&content, 10000000).to_string()
}

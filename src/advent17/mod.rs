mod algo;
mod domain;

use crate::advent17::algo::*;

use crate::helper::read_puzzle_input;

pub fn advent17() -> String {
    let content = read_puzzle_input("./src/advent17/rocks_test.txt");
    start(&content);
    "hallo".into()
}

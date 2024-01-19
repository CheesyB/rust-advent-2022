mod algo;
mod domain;

use crate::advent17::algo::*;

use crate::helper::read_puzzle_input;

pub fn advent17() -> String {
    let content = read_puzzle_input("./src/advent17/rocks.txt");
    let mut rested_shapes = vec![];
    simulate(&content, &mut rested_shapes, 2022).to_string()
}

pub fn advent17_2() -> String {
    const DISTINCT_ITERERATIONS: usize = 50455;
    const DISTICT_HEIGHT: i64 = 78498; // math
    const REMAINING_HIGHT: i64 = 5; // simualte
    const CYCLES: i64 = 1981964; // 1000000000000 / UNIQUE_CYCLE_HEIGHT
    const REMAINING_ITERATION: usize = 13345; // 1000000000000 % UNIQUE_CYCLE_HEIGHT

    let content = read_puzzle_input("./src/advent17/rocks.txt");
    let mut rested_shapes = vec![];
    let distict_height = simulate(&content, &mut rested_shapes, DISTINCT_ITERERATIONS);
    let remaining_height = simulate(&content, &mut rested_shapes, REMAINING_ITERATION);
    print!("dh: {}, rh:{}", distict_height, remaining_height);
    let result = distict_height * CYCLES + remaining_height;
    // to low 155580309341
    result.to_string()
}

mod ele_algo;
mod ele_domain;
mod parsing;

use crate::advent16::ele_algo::*;
use crate::advent16::parsing::*;

use crate::helper;

pub fn advent16() -> String {
    let content = helper::read_puzzle_input("./src/advent16/pressure.txt");
    let mut input = vec![];
    for line in content.lines() {
        let (_, junk) = parse_line(line).ok().unwrap();
        input.push(junk);
    }
    let network: crate::advent16::ele_domain::Network =
        crate::advent16::ele_domain::Network::new(input);

    let pressure_map = agent_bfs(&network, "AA", 30);
    pressure_map.iter().map(|p| p.1).max().unwrap().to_string()
}

pub fn advent16_2() -> String {
    let content = helper::read_puzzle_input("./src/advent16/pressure.txt");
    let mut input = vec![];
    for line in content.lines() {
        let (_, junk) = parse_line(line).ok().unwrap();
        input.push(junk);
    }
    let network: crate::advent16::ele_domain::Network =
        crate::advent16::ele_domain::Network::new(input);
    two_agent_bfs(&network, "AA", 26).to_string()
}

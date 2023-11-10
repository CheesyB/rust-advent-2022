mod algo;
mod domain;
mod parsing;

use crate::advent16::algo::*;
use crate::advent16::domain::*;
use crate::advent16::parsing::*;

use crate::helper;

pub fn advent16() -> String {
    let content = helper::read_puzzle_input("./src/advent16/pressure.txt");
    let mut input = vec![];
    for line in content.lines() {
        let (_, junk) = parse_line(line).ok().unwrap();
        input.push(junk);
    }
    let network: Network = Network::new(input);
    let result = start_explore(&network);
    let max_score = result.iter().map(|route| route.score(30)).max().unwrap();
    // to high 8680
    max_score.to_string()
}

pub fn advent16_2() -> String {
    let content = helper::read_puzzle_input("./src/advent16/pressure_test.txt");
    let mut input = vec![];
    for line in content.lines() {
        let (_, junk) = parse_line(line).ok().unwrap();
        input.push(junk);
    }
    let network: Network = Network::new(input);
    start_ele_explore(&network).to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_path() {
        let content = helper::read_puzzle_input("./src/advent16/pressure_test.txt");
        let mut input = vec![];
        for line in content.lines() {
            let (_, junk) = parse_line(line).ok().unwrap();
            input.push(junk);
        }
        let network: Network = Network::new(input);
        let start = Route::new(['A', 'A']);
        ele_bfs(&network, start);
        assert_eq!(1.0, 1707 as f32);
    }

    #[test]
    fn test_parse() {
        let mut route = Route::new(['A', 'A']);
        route.open_valve(2, 20);
        route.open_valve(5, 13);
        let score = route.score(6);
        assert_eq!(score, 93);
    }
}

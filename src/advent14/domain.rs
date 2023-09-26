use std::num::ParseIntError;

use nom::IResult;
use nom::character::complete::{digit1, char};
use nom::bytes::complete::tag;
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::{tuple, separated_pair};
use nom::character::complete::u32;
pub enum Fill {
    Air,
    Rock,
    FallingSand,
    RestedSand,
}

pub type Grid = Vec<Vec<Fill>>;

pub struct Map {
    grid: Grid,
}

struct Coords(usize,usize);

impl Map {
    pub fn new(input: &str) -> () {
        for 
    }

    fn parse_rocks(line: &str) -> Vec<Coords>{
        let temp= separated_pair(u32, char(','), u32);
        let (rest, raw_coords) = separated_list1(tag(" -> "), temp )(line).ok().unwrap();
        let coords = raw_coords.iter().map(|c| Coords(c.0 as usize,c.1 as usize)).collect();
        
        coords
    }
}

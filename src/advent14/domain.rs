use std::fmt::Display;
use std::vec;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::u32;
use nom::error::ErrorKind;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

#[derive(Debug, Clone)]
pub enum Fill {
    Air,
    Rock,
    FallingSand,
    RestedSand,
}

impl Display for Fill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fill::Air => write!(f, "."),
            Fill::Rock => write!(f, "#"),
            Fill::FallingSand => write!(f, "ⵔ"),
            Fill::RestedSand => write!(f, "ⵙ"),
        }
    }
}

pub type Grid = Vec<Vec<Fill>>;

#[derive(Debug, Clone)]
pub struct Map {
    grid: Grid,
}

struct Coord(usize, usize);

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for col in row {
                write!(f, "{} ", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn new(input: &str) -> Map {
        let mut map: Map = Map {
            grid: vec![vec![Fill::Air; 600]; 600],
        };
        let mut coords: Vec<Coord> = vec![];
        for line in input.lines() {
            let mut tmp = Self::rock_coords(Self::parse_rocks(line));
            coords.append(&mut tmp);
        }
        for coord in coords {
            map.grid[coord.0][coord.1] = Fill::Rock;
        }
        map
    }

    fn rock_coords(rock_edges: Vec<Coord>) -> Vec<Coord> {
        let mut rocks = vec![];
        for edge in rock_edges.windows(2) {
            let mut tmp = &mut Self::expand_edges_to_line(&edge[0], &edge[1]);
            rocks.append(&mut tmp);
        }
        rocks
    }

    fn expand_edges_to_line(first_edge: &Coord, second_edge: &Coord) -> Vec<Coord> {
        let delta_0 = first_edge.0..second_edge.0;
        let delta_1 = first_edge.1..second_edge.1;
        let mut line = vec![];
        for x in delta_0.clone() {
            for y in delta_1.clone() {
                line.push(Coord(x, y));
            }
        }
        line
    }

    fn parse_rocks(line: &str) -> Vec<Coord> {
        let raw = separated_list1(
            tag(" -> "),
            separated_pair(u32::<&str, (&str, ErrorKind)>, char(','), u32),
        )(line);
        let coords = raw
            .map(|(_, coords)| {
                coords
                    .iter()
                    .map(|c| Coord(c.0 as usize, c.1 as usize))
                    .collect::<Vec<Coord>>()
            })
            .ok()
            .unwrap();

        print!("here");
        coords
    }
}

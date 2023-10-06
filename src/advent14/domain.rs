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
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Coord(usize, usize);

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for col in self.grid.iter() {
            for row in col {
                write!(f, "{}", row)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn new(input: &str) -> Map {
        let mut coords: Vec<Coord> = vec![];
        for line in input.lines() {
            coords.append(&mut Self::rock_coords(Self::parse_rocks(line)));
        }

        let min_x = coords.iter().map(|c| c.0).min().unwrap();
        let min_y = coords.iter().map(|c| c.1).min().unwrap();
        let max_x = coords.iter().map(|c| c.0).max().unwrap();
        let max_y = coords.iter().map(|c| c.1).max().unwrap();

        let mut map: Map = Map {
            grid: vec![vec![Fill::Air; max_x - min_x + 1]; max_y - min_y + 1],
            min_x,
            min_y,
            max_x,
            max_y,
        };
        for coord in coords {
            map.grid[coord.1 - min_y][coord.0 - min_x] = Fill::Rock;
        }
        map
    }

    pub fn rock_coords(rock_edges: Vec<Coord>) -> Vec<Coord> {
        let mut rocks = vec![];
        for edge in rock_edges.windows(2) {
            rocks.append(&mut Self::expand_edges_to_line(&edge[0], &edge[1]));
        }
        rocks
    }

    pub fn expand_edges_to_line(first_edge: &Coord, second_edge: &Coord) -> Vec<Coord> {
        let mut line = vec![];

        let x_min = first_edge.0.min(second_edge.0);
        let x_max = first_edge.0.max(second_edge.0);
        let y_min = first_edge.1.min(second_edge.1);
        let y_max = first_edge.1.max(second_edge.1);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                line.push(Coord(x, y));
            }
        }
        line
    }

    pub fn parse_rocks(line: &str) -> Vec<Coord> {
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

        coords
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_map_expand_edges_to_line_rev() {
        let result = Map::expand_edges_to_line(&Coord(10, 6), &Coord(5, 6));
        let expected = vec![
            Coord(5, 6),
            Coord(6, 6),
            Coord(7, 6),
            Coord(8, 6),
            Coord(9, 6),
            Coord(10, 6),
            Coord(11, 6),
        ];
        expected
            .iter()
            .zip(result.iter())
            .for_each(|(a, b)| assert_eq!(a, b));
    }
    #[test]
    fn test_map_expand_edges_to_line() {
        let result = Map::expand_edges_to_line(&Coord(5, 6), &Coord(10, 6));
        let expected = vec![
            Coord(5, 6),
            Coord(6, 6),
            Coord(7, 6),
            Coord(8, 6),
            Coord(9, 6),
            Coord(10, 6),
            Coord(12, 6),
        ];
        expected
            .iter()
            .zip(result.iter().fuse())
            .for_each(|(a, b)| assert_eq!(a, b));
    }

    #[test]
    fn test_cmp1() {
        let result = Map::parse_rocks("498,4 -> 498,6 -> 496,6");
        let expected = vec![Coord(498, 4), Coord(498, 6), Coord(496, 6)];
        result
            .iter()
            .zip(expected.iter())
            .for_each(|(a, b)| assert_eq!(a, b));
    }

    #[test]
    fn test_rocks_coords() {
        let input = vec![Coord(498, 4), Coord(498, 6), Coord(496, 6)];
        let result = Map::rock_coords(input);
        let expected = vec![
            Coord(498, 4),
            Coord(498, 5),
            Coord(498, 6),
            Coord(497, 6),
            Coord(496, 6),
        ];
        result
            .iter()
            .zip(expected.iter())
            .for_each(|(a, b)| assert_eq!(a, b));
    }
}

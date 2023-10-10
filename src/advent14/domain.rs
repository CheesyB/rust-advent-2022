use std::fmt::Display;
use std::vec;

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::u32;
use nom::error::ErrorKind;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

pub const START_POSITION: Coord = Coord(500, 0);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fill {
    Air,
    Rock,
    RestedSand,
}

impl Display for Fill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fill::Air => write!(f, "."),
            Fill::Rock => write!(f, "#"),
            Fill::RestedSand => write!(f, "X"),
        }
    }
}

pub type Grid = Vec<Vec<Fill>>;

#[derive(Debug, Clone)]
pub struct Map {
    grid: Grid,
    min_x: usize,
    max_y: usize,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Coord(usize, usize);

enum Moves {
    DOWN,
    LEFT,
    RIGHT,
}

impl Coord {
    fn move_to(&self, mov: Moves) -> Option<Coord> {
        match mov {
            Moves::DOWN => Some(Coord(self.0, self.1 + 1)),
            Moves::LEFT => Some(Coord(self.0 - 1, self.1 + 1)),
            Moves::RIGHT => Some(Coord(self.0 + 1, self.1 + 1)),
        }
    }
}

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

fn simulate_sand_corn_until_full(map: &mut Map, start_position: Coord) -> bool {
    let mut next_position = start_position.clone();
    let mut previouse_position = start_position.clone();
    loop {
        previouse_position = next_position.clone();
        if let Some(new_position) = sand_fall_next(map, next_position.clone()) {
            next_position = new_position.clone();
            continue;
        }
        break;
    }
    map.fill_at(previouse_position.clone(), Fill::RestedSand);
    if previouse_position == Coord(500,0) {
        return false;
    } else {
        return true;
    }
}
pub fn simulate_unitl_full(map: &mut Map, start_position: &Coord) -> i32 {
    let mut count = 1;
    while simulate_sand_corn_until_full(map, start_position.clone()) {
        count += 1;
    }
    count
}
pub fn simulate_void(map: &mut Map, start_position: &Coord) -> i32 {
    let mut count = 0;
    while simulate_sand_corn_void(map, start_position.clone()) {
        count += 1;
    }
    count
}

fn simulate_sand_corn_void(map: &mut Map, start_position: Coord) -> bool {
    let mut next_position = start_position.clone();
    let mut previouse_position = start_position.clone();
    loop {
        previouse_position = next_position.clone();
        if let Some(new_position) = sand_fall_next(map, next_position.clone()) {
            next_position = new_position.clone();
            //falls into the void 
            if new_position.1 > map.max_y {
                return false;
            }
            continue;
        }
        break;
    }
    map.fill_at(previouse_position.clone(), Fill::RestedSand);
    true
}

fn sand_fall_next(map: &Map, sand_position: Coord) -> Option<Coord> {
    let sand_pos = [
        sand_position.move_to(Moves::DOWN)?,
        sand_position.move_to(Moves::LEFT)?,
        sand_position.move_to(Moves::RIGHT)?,
    ];
    if map.get_fill_at(&sand_pos[0]) == Fill::Air {
        return Some(sand_pos[0]);
    }
    if map.get_fill_at(&sand_pos[1]) == Fill::Air {
        return Some(sand_pos[1]);
    }
    if map.get_fill_at(&sand_pos[2]) == Fill::Air {
        return Some(sand_pos[2]);
    }
    None
}

impl Map {
    const PADDING: usize = 200;

    pub fn new(input: &str) -> Map {
        let mut coords: Vec<Coord> = vec![];
        for line in input.lines() {
            coords.append(&mut Self::rock_coords(Self::parse_rocks(line)));
        }

        let min_x = coords.iter().map(|c| c.0).min().unwrap();
        let min_y = coords.iter().map(|c| c.1).min().unwrap();
        let max_x = coords.iter().map(|c| c.0).max().unwrap();
        let max_y = coords.iter().map(|c| c.1).max().unwrap();

        coords.append(&mut Self::rock_coords(vec![
            Coord(min_x - Self::PADDING + 5,  max_y + 2),
            Coord(max_x + Self::PADDING -5 ,  max_y + 2),
        ]));

        let mut map: Map = Map {
            grid: vec![
                vec![Fill::Air; Self::PADDING + max_x - min_x + Self::PADDING ]; // 100 due to bug somewhere
                Self::PADDING + max_y - min_y 
            ],
            min_x,
            max_y,
        };
        for coord in coords {
            map.fill_at(coord, Fill::Rock);
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

    pub fn get_fill_at(&self, pos: &Coord) -> Fill {
        self.grid[pos.1][ Self::PADDING + pos.0 - self.min_x]
    }

    pub fn fill_at(&mut self, pos: Coord, fill: Fill) {
        self.grid[pos.1][Self::PADDING + pos.0 - self.min_x] = fill;
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

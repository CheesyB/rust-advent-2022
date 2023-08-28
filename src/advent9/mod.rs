use std::{fs, ops, str::FromStr};

fn basic() -> String {
    let file_path = "./src/advent9/rope.txt";
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    R,
    L,
    U,
    D,
}

impl Direction {
    pub fn get(self) -> Position {
        match self {
            Direction::R => Position(1, 0),
            Direction::L => Position(-1, 0),
            Direction::U => Position(0, 1),
            Direction::D => Position(0, -1),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::R),
            "L" => Ok(Direction::L),
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            &_ => Err(()),
        }
    }
}

// x, y
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct Position(i32, i32);

impl ops::Add for Position {
    type Output = Position;
    fn add(self, rhs: Self) -> Position {
        return Position(self.0 + rhs.0, self.1 + rhs.1);
    }
}

impl ops::Sub for Position {
    type Output = Position;
    fn sub(self, rhs: Self) -> Position {
        return Position(self.0 - rhs.0, self.1 - rhs.1);
    }
}

fn tail_follow(head: Position, tail: Position) -> Position {
    let rel_pos = head - tail;
    match rel_pos {
        Position(0, 0) => tail.clone(),
        Position(1, 1) => tail.clone(),
        Position(1, 0) => tail.clone(),
        Position(1, -1) => tail.clone(),
        Position(0, -1) => tail.clone(),
        Position(-1, -1) => tail.clone(),
        Position(-1, 0) => tail.clone(),
        Position(-1, 1) => tail.clone(),
        Position(0, 1) => tail.clone(),

        Position(1, 2) => tail + Position(1, 1),
        Position(0, 2) => tail + Position(0, 1),
        Position(-1, 2) => tail + Position(-1, 1),

        Position(2, 1) => tail + Position(1, 1),
        Position(2, 0) => tail + Position(1, 0),
        Position(2, -1) => tail + Position(1, -1),

        Position(1, -2) => tail + Position(1, -1),
        Position(0, -2) => tail + Position(0, -1),
        Position(-1, -2) => tail + Position(-1, -1),

        Position(-2, -1) => tail + Position(-1, -1),
        Position(-2, 0) => tail + Position(-1, 0),
        Position(-2, 1) => tail + Position(-1, 1),

        Position(2, 2) => tail + Position(1, 1),
        Position(2, -2) => tail + Position(1, -1),
        Position(-2, 2) => tail + Position(-1, 1),
        Position(-2, -2) => tail + Position(-1, -1),

        pos => panic!("{:?}: wrong position", pos),
    }
}

fn parse_line(input: &str) -> (Direction, u32) {
    let mut split = input.split_whitespace();
    (
        Direction::from_str(split.next().unwrap()).unwrap(),
        u32::from_str_radix(split.next().unwrap(), 10).unwrap(),
    )
}

#[allow(dead_code)]
fn print_pos(input: &Vec<Position>) {
    let mut grid = vec![vec!['.'; 40 as usize]; 40 as usize];
    input
        .iter()
        .for_each(|pos| grid[(20 + pos.0) as usize][(20 + pos.1) as usize] = '#');
    for i in grid.iter() {
        for j in i.iter() {
            print!("{} ", j);
        }
        print!("\n")
    }
}

fn iter_tail(head_pos: Vec<Position>) -> Vec<Position> {
    let mut visited_pos = vec![];
    let mut tail = Position(0, 0);
    visited_pos.push(tail);
    for head in head_pos {
        tail = tail_follow(head, tail);
        visited_pos.push(tail);
    }
    return visited_pos;
}

pub fn advent9_2() -> String {
    let moves = basic();
    let mut head_pos = vec![];
    let mut head = Position(0, 0);
    let mut tail = Position(0, 0);
    head_pos.push(tail);
    for line in moves.lines() {
        let (dir, step_count) = parse_line(line);
        for _ in 0..step_count {
            head = head + dir.get();
            tail = tail_follow(head, tail);
            head_pos.push(tail);
        }
    }
    for _ in 0..8 {
        head_pos = iter_tail(head_pos);
    }
    head_pos.sort_unstable();
    head_pos.dedup();
    return head_pos.len().to_string();
}

pub fn advent9() -> String {
    let moves = basic();
    let mut visited_pos = vec![];
    let mut head = Position(0, 0);
    let mut tail = Position(0, 0);
    visited_pos.push(tail);
    for line in moves.lines() {
        let (dir, step_count) = parse_line(line);
        for _ in 0..step_count {
            head = head + dir.get();
            tail = tail_follow(head, tail);
            visited_pos.push(tail);
        }
    }
    visited_pos.sort_unstable();
    visited_pos.dedup();
    return visited_pos.len().to_string();
}

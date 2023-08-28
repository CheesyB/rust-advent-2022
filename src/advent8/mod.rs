use std::{cell::RefCell, fs, vec};

fn basic() -> String {
    let file_path = "./src/advent8/trees.txt";
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

#[derive(Debug)]
struct Tree {
    val: u32,
    score: Vec<u32>,
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl Tree {
    pub fn new(val: u32) -> Tree {
        Tree {
            val,
            score: vec![],
            north: true,
            east: true,
            south: true,
            west: true,
        }
    }
    fn score(&mut self, grid: &Vec<Vec<RefCell<Tree>>>, row: usize, col: usize) {
        //NORTH
        for (score, i) in (0..row).rev().enumerate() {
            if self.val <= grid[i][col].borrow().val || i == 0 {
                self.score.push(score as u32 + 1);
                break;
            }
        }
        //EAST
        for (score, i) in (col + 1..grid[row].len()).enumerate() {
            if self.val <= grid[row][i].borrow().val || i == grid[row].len() - 1 {
                self.score.push(score as u32 + 1);
                break;
            }
        }
        //SOUTH
        for (score, i) in (row + 1..grid.len()).enumerate() {
            if self.val <= grid[i][col].borrow().val || i == grid.len() - 1 {
                self.score.push(score as u32 + 1);
                break;
            }
        }
        //WEST
        for (score, i) in (0..col).rev().enumerate() {
            if self.val <= grid[row][i].borrow().val || i == 0 {
                self.score.push(score as u32 + 1);
                break;
            }
        }
    }

    fn gaze(&mut self, grid: &Vec<Vec<RefCell<Tree>>>, row: usize, col: usize) {
        //NORTH
        for i in (0..row).rev() {
            if self.val <= grid[i][col].borrow().val {
                self.north = false;
            }
        }
        //EAST
        for i in col + 1..grid[row].len() {
            if self.val <= grid[row][i].borrow().val {
                self.east = false;
            }
        }
        //SOUTH
        for i in row + 1..grid.len() {
            if self.val <= grid[i][col].borrow().val {
                self.south = false;
            }
        }
        //WEST
        for i in 0..col {
            if self.val <= grid[row][i].borrow().val {
                self.west = false;
            }
        }
    }
    pub fn is_visible(&self) -> bool {
        self.north || self.east || self.south || self.west
    }
}

pub fn advent8_2() -> String {
    let content = basic();
    let mut grid = vec![];
    for (count, line) in content.lines().enumerate() {
        grid.push(vec![]);
        for char in line.chars() {
            grid[count].push(RefCell::new(Tree::new(char.to_digit(10).unwrap())));
        }
    }
    for row in 1..grid.len() - 1 {
        for col in 1..grid[row].len() - 1 {
            grid[row][col].borrow_mut().score(&grid, row, col);
        }
    }

    let max = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|tree| {
                    tree.borrow()
                        .score
                        .iter()
                        .copied()
                        .reduce(|x, y| x * y)
                        .unwrap_or_default()
                })
                .max()
        })
        .max()
        .flatten()
        .unwrap();

    return max.to_string();
}

pub fn advent8() -> String {
    let content = basic();
    let mut grid = vec![];
    for (count, line) in content.lines().enumerate() {
        grid.push(vec![]);
        for char in line.chars() {
            grid[count].push(RefCell::new(Tree::new(char.to_digit(10).unwrap())));
        }
    }
    for row in 1..grid.len() {
        for col in 1..grid[row].len() {
            grid[row][col].borrow_mut().gaze(&grid, row, col);
        }
    }
    let mut visible_trees = 0;

    for row in grid.iter() {
        for tree in row.iter() {
            if tree.borrow().is_visible() {
                visible_trees += 1;
            }
        }
    }

    return visible_trees.to_string();
}

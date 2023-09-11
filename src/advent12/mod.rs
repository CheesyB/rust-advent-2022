use std::{cell::RefCell, rc::Rc};

use crate::{
    advent12::tree::{breath_first, get_path_length, print_node, Position},
    helper,
};

mod tree {
    use std::{
        cell::RefCell,
        collections::VecDeque,
        convert::identity,
        rc::{Rc, Weak},
        vec,
    };

    use super::{Grid, Visited};

    #[derive(Debug)]
    pub struct Node {
        position: Position,
        val: char,
        children: Vec<Rc<RefCell<Node>>>,
        parent: Option<Weak<RefCell<Node>>>,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Position {
        pub row: usize,
        pub col: usize,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Direction {
        NORTH,
        EAST,
        SOUTH,
        WEST,
    }

    impl Position {
        pub fn moving(self, dir: Direction) -> Option<Position> {
            match dir {
                Direction::NORTH => {
                    if self.row == 0 {
                        return None;
                    }
                    Some(Position {
                        row: self.row - 1,
                        col: self.col,
                    })
                }
                Direction::EAST => Some(Position {
                    row: self.row,
                    col: self.col + 1,
                }),
                Direction::SOUTH => Some(Position {
                    row: self.row + 1,
                    col: self.col,
                }),
                Direction::WEST => {
                    if self.col == 0 {
                        return None;
                    }
                    Some(Position {
                        row: self.row,
                        col: self.col - 1,
                    })
                }
            }
        }
    }

    impl Node {
        pub fn new(position: Position, val: char) -> Node {
            Node {
                position,
                val,
                children: vec![],
                parent: None,
            }
        }
        pub fn new_in_dir(&self, dir: Direction, grid: &Vec<Vec<char>>) -> Option<Node> {
            Some(Node {
                position: self.position.moving(dir)?,
                val: grid[self.position.moving(dir)?.row][self.position.moving(dir)?.col],
                children: vec![],
                parent: None,
            })
        }
        fn add_child<'b>(&mut self, child: Rc<RefCell<Node>>) {
            self.children.push(child)
        }

        pub fn get_parent(&self) -> Rc<RefCell<Node>> {
            self.parent.as_ref().unwrap().upgrade().unwrap()
        }

        fn can_reach(&self, next: char) -> bool {
            if (self.val as u8) <= next as u8 + 1 {
                return true;
            }
            if next == 'E' && (self.val == 'y' || self.val == 'z') {
                return true;
            }
            if (next == 'a' || next == 'b') && self.val == 'S' {
                return true;
            }
            false
        }
    }

    fn check_boundary(
        grid: &Vec<Vec<char>>,
        pos: Position,
        visited: &Visited,
        parent: &Rc<RefCell<Node>>,
    ) -> bool {
        grid.get(pos.row).is_some()
            && grid[pos.row].get(pos.col).is_some()
            && visited[pos.row][pos.col] == false
            && parent.borrow().can_reach(grid[pos.row][pos.col])
    }

    fn try_extend_in_direction(
        parent: &Rc<RefCell<Node>>,
        grid: &Vec<Vec<char>>,
        visited: &Visited,
        direction: Direction,
    ) -> Option<Rc<RefCell<Node>>> {
        let pos = parent.as_ref().borrow().position;
        if check_boundary(grid, pos.moving(direction)?, visited, parent) {
            let child = Rc::new(RefCell::new(
                parent.as_ref().borrow().new_in_dir(direction, grid)?,
            ));
            add_child(parent, &child);
            return Some(child);
        }
        None
    }

    pub fn get_adjacent_pos(
        parent: &Rc<RefCell<Node>>,
        grid: &Grid,
        visited: &Visited,
    ) -> Vec<Rc<RefCell<Node>>> {
        vec![
            try_extend_in_direction(parent, grid, visited, Direction::NORTH),
            try_extend_in_direction(parent, grid, visited, Direction::EAST),
            try_extend_in_direction(parent, grid, visited, Direction::SOUTH),
            try_extend_in_direction(parent, grid, visited, Direction::WEST),
        ]
        .into_iter()
        .filter_map(identity)
        .collect()
    }

    pub fn add_child(parent: &Rc<RefCell<Node>>, child: &Rc<RefCell<Node>>) {
        child.as_ref().borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.as_ref().borrow_mut().add_child(Rc::clone(&child));
    }
    pub fn breath_first(root: Rc<RefCell<Node>>, grid: &Grid) -> Rc<RefCell<Node>> {
        let mut stack = VecDeque::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        visited[root.as_ref().borrow().position.row][root.as_ref().borrow().position.col] = true;
        stack.extend(get_adjacent_pos(&root, grid, &visited));
        consume_stack(&mut stack, grid, &mut visited).unwrap()
    }
    pub fn get_path_length(mut end_node: Rc<RefCell<Node>>) -> usize {
        let mut path_count: usize = 0;
        loop {
            println!("{} ", path_count);
            if end_node.as_ref().borrow().val == 'S' {
                break;
            }
            let _tmp = &end_node
                .as_ref()
                .borrow()
                .parent
                .as_ref()
                .unwrap()
                .clone()
                .upgrade()
                .unwrap();
            let tmp2 = _tmp.upgrade().unwrap();
            end_node = tmp2;
            path_count += 1;
        }
        path_count
    }

    fn consume_stack(
        stack: &mut VecDeque<Rc<RefCell<Node>>>,
        grid: &Grid,
        visited: &mut Visited,
    ) -> Option<Rc<RefCell<Node>>> {
        while let Some(node) = stack.pop_front() {
            let new_nodes = get_adjacent_pos(&node, grid, &visited);
            for node in new_nodes.iter() {
                visited[node.as_ref().borrow().position.row][node.as_ref().borrow().position.col] =
                    true;
                print!("{}, ", node.borrow().val);
            }
            if node.borrow().val == 'E' {
                println!("Found End");
                return Some(node);
            }
            stack.extend(new_nodes.into_iter());
            println!();
        }
        println!("End not found");
        None
    }
    #[warn(dead_code)]
    pub fn print_node(node: Rc<RefCell<Node>>) {
        let binding = Rc::try_unwrap(node).unwrap_or_else(|_| panic!("not so good"));
        let node = binding.into_inner();
        dbg!(&node);
    }
}

type Grid = Vec<Vec<char>>;
type Visited = Vec<Vec<bool>>;

pub fn advent12() -> String {
    use tree::Node;

    let content = helper::read_puzzle_input("./src/advent12/elevation.txt");
    let mut grid: Vec<Vec<char>> = vec![vec![]];
    let mut start = Position { row: 0, col: 0 };

    for (row, line) in content.lines().enumerate() {
        grid.push(vec![]);
        for (col, ch) in line.chars().enumerate() {
            grid[row].push(ch);
            if ch == 'S' {
                start = Position { row, col }
            }
        }
    }
    let root = Rc::new(RefCell::new(Node::new(start, 'S'))); // use a here
    let end_node = breath_first(root, &grid);

    get_path_length(end_node).to_string()
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::advent12::tree::{add_child, Node};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::{tree::print_node, *};

    #[test]
    fn test_tree() {
        let root = Rc::new(RefCell::new(Node::new(Position { row: 0, col: 0 }, 'S')));
        let child = Rc::new(RefCell::new(Node::new(Position { row: 0, col: 0 }, 'T')));
        add_child(&root, &child);
        print_node(root);
        println!("Thing");
        assert!(false);
    }
}

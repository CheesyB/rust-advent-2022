use std::{cell::RefCell, convert::identity, rc::Rc, vec};

use crate::{
    advent12::tree::{breath_first, capsule_bfs, get_path, print_positions, Position},
    helper,
};

mod tree {
    use crate::helper;
    use std::{
        cell::RefCell,
        collections::VecDeque,
        convert::identity,
        fs,
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

        // why does it only work when I unwrap the parent inside the impl?
        pub fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
            if self.parent.as_ref().is_none() {
                return None;
            }
            self.parent.as_ref().unwrap().upgrade()
        }

        fn can_reach(&self, next: char) -> bool {
            if (self.val as u8 + 1) >= next as u8 && next != 'E' {
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
        parent.as_ref().borrow_mut().add_child(Rc::clone(child));
    }
    pub fn breath_first(root: &Rc<RefCell<Node>>, grid: &Grid) -> Option<Rc<RefCell<Node>>> {
        let mut stack = VecDeque::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        visited[root.as_ref().borrow().position.row][root.as_ref().borrow().position.col] = true;
        stack.extend(get_adjacent_pos(&root, grid, &visited));
        consume_stack(&mut stack, grid, &mut visited)
    }
    pub fn get_path(mut end_node: Rc<RefCell<Node>>) -> Vec<Position> {
        let mut positions = vec![];
        positions.push(end_node.borrow().position.clone());
        loop {
            if end_node.as_ref().borrow().val == 'S' {
                break;
            }
            let tmp = end_node.borrow_mut().get_parent();
            if tmp.is_none() {
                break;
            }
            let un_tmp = tmp.unwrap();
            positions.push(un_tmp.borrow().position.clone());
            end_node = un_tmp;
        }
        positions
    }
    pub fn capsule_bfs(start_position: &Position, grid: &Grid) -> Option<usize> {
        let root = Rc::new(RefCell::new(Node::new(start_position.clone(), 'a')));
        let end_node = breath_first(&root, grid);
        if end_node.is_some() {
            let path = get_path(end_node.unwrap());
            return Some(path.len() - 1);
        }
        None
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
            }
            if node.borrow().val == 'E' {
                return Some(node);
            }
            stack.extend(new_nodes.into_iter());
        }
        //println!("End not found");
        None
    }
    #[allow(dead_code)]
    pub fn print_node(node: Rc<RefCell<Node>>) {
        let binding = Rc::try_unwrap(node).unwrap_or_else(|_| panic!("not so good"));
        let node = binding.into_inner();
        dbg!(&node);
    }
    pub fn print_positions(positions: &Vec<Position>) {
        let mut buff = String::new();
        let content = helper::read_puzzle_input("./src/advent12/elevation.txt");
        let mut grid: Vec<Vec<char>> = vec![vec![]];

        for (row, line) in content.lines().enumerate() {
            grid.push(vec![]);
            for ch in line.chars() {
                grid[row].push(ch);
            }
        }
        for pos in positions.into_iter() {
            grid[pos.row][pos.col] = '#';
        }
        for row in grid.iter() {
            for col in row.iter() {
                buff.push_str(col.to_string().as_str());
            }
            buff.push_str("\n".to_string().as_str());
        }
        fs::write("./src/advent12/path.txt", buff.to_string()).expect("no damage");
    }
}

type Grid = Vec<Vec<char>>;
type Visited = Vec<Vec<bool>>;

pub fn advent12_2() -> String {
    let content = helper::read_puzzle_input("./src/advent12/elevation.txt");
    let mut grid: Vec<Vec<char>> = vec![vec![]];
    let mut start_positions = vec![];

    for (row, line) in content.lines().enumerate() {
        grid.push(vec![]);
        for (col, ch) in line.chars().enumerate() {
            grid[row].push(ch);
            if ch == 'S' || ch == 'a' {
                start_positions.push(Position { row, col });
            }
        }
    }
    let mut all_paths_from_a: Vec<Option<usize>> = vec![];
    for (_count, pos) in start_positions.iter().enumerate() {
        //println!("{}: {:?}", _count, pos);
        let path_len = capsule_bfs(pos, &grid);
        all_paths_from_a.push(path_len);
    }
    all_paths_from_a
        .into_iter()
        .filter_map(identity)
        .min()
        .unwrap()
        .to_string()
}
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
    let end_node = breath_first(&root, &grid);

    let path = get_path(end_node.unwrap());
    print_positions(&path);
    (path.len() - 1).to_string() // -1 because the 'E' does not count towards the path length
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
        assert!(true);
    }
}

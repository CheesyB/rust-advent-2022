use super::point::{Point, ONE_DOWN, ONE_LEFT, ONE_RIGHT};

pub enum Direction {
    LEFT,
    RIGHT,
    DOWN,
}

impl Direction {
    pub fn value(self) -> &'static Point {
        match self {
            Direction::LEFT => ONE_LEFT,
            Direction::RIGHT => ONE_RIGHT,
            Direction::DOWN => ONE_DOWN,
        }
    }
}

impl From<&char> for Direction {
    fn from(value: &char) -> Self {
        match *value {
            '>' => Self::RIGHT,
            '<' => Self::LEFT,
            _ => panic!("wrong direction conversion char"),
        }
    }
}

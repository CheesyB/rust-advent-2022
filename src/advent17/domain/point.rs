use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Sub;

pub const ONE_LEFT: &'static Point = &Point::new(-1, 0);
pub const ONE_RIGHT: &'static Point = &Point::new(1, 0);
pub const ONE_DOWN: &'static Point = &Point::new(0, -1);

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.y > other.y {
            return Ordering::Greater;
        }
        if self.y < other.y {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

impl Sub<&Point> for Point {
    type Output = Self;
    fn sub(self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Self;
    fn add(self, other: &Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

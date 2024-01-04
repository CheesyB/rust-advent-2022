use std::ops::Add;
use std::ops::Sub;

const PLUS: Shape = Shape::new(&[
    Point::new(1, 0),
    Point::new(0, 1),
    Point::new(1, 1),
    Point::new(2, 1),
    Point::new(1, 2),
]);
const MINUS: Shape = Shape::new(&[
    Point::new(0, 0),
    Point::new(1, 0),
    Point::new(2, 0),
    Point::new(3, 0),
]);

const L: Shape = Shape::new(&[
    Point::new(0, 0),
    Point::new(1, 0),
    Point::new(2, 0),
    Point::new(2, 1),
    Point::new(2, 2),
]);

const I: Shape = Shape::new(&[
    Point::new(0, 0),
    Point::new(0, 1),
    Point::new(0, 2),
    Point::new(0, 3),
]);
const CUBE: Shape = Shape::new(&[
    Point::new(0, 0),
    Point::new(0, 1),
    Point::new(1, 0),
    Point::new(1, 1),
]);

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Shape {
    rel_pts: &'static [Point],
    ref_pt: Point,
}

impl Shape {
    pub const fn new(relative_pts: &'static [Point]) -> Shape {
        Shape {
            rel_pts: relative_pts,
            ref_pt: Point::new(0, 0),
        }
    }
    pub fn shift(&mut self, move_to: &Point) {
        self.ref_pt = move_to + self.ref_pt;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Shapes {
    Plus,
    Minus,
    L,
    I,
    Cube,
}

impl Shapes {
    pub fn value(&self) -> &Shape {
        match self {
            Shapes::Plus => &PLUS,
            Shapes::Minus => &MINUS,
            Shapes::L => &L,
            Shapes::I => &I,
            Shapes::Cube => &CUBE,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: &Point) -> <Self as Add<Point>>::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: &Point) -> <Self as Add<Point>>::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

use crate::advent17::domain::point::Point;
use crate::advent17::domain::shape::*;

pub const SHAPE_ORDER: &'static [Shape; 5] = &[MINUS, PLUS, L, I, CUBE];

pub const PLUS: Shape = Shape::new(
    &[
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(2, 1),
        Point::new(1, 2),
    ],
    3,
);
pub const MINUS: Shape = Shape::new(
    &[
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(3, 0),
    ],
    1,
);

pub const L: Shape = Shape::new(
    &[
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(2, 1),
        Point::new(2, 2),
    ],
    3,
);

pub const I: Shape = Shape::new(
    &[
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
    ],
    4,
);
pub const CUBE: Shape = Shape::new(
    &[
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(1, 0),
        Point::new(1, 1),
    ],
    2,
);

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

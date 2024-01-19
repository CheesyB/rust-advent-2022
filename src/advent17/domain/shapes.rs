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
    '+',
);
pub const MINUS: Shape = Shape::new(
    &[
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(3, 0),
    ],
    1,
    '-',
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
    'L',
);

pub const I: Shape = Shape::new(
    &[
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
    ],
    4,
    'I',
);
pub const CUBE: Shape = Shape::new(
    &[
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(1, 0),
        Point::new(1, 1),
    ],
    2,
    'C',
);

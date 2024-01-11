use crate::advent17::domain::point::Point;

use super::direction::Direction;

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

    pub fn get_position(&self) -> Vec<Point> {
        self.rel_pts.iter().map(|pt| self.ref_pt + pt).collect()
    }

    pub fn get_max_hight(&self) -> i32 {
        self.get_position().iter().max().unwrap().y
    }

    pub fn shift(&mut self, move_to: &Point) {
        self.ref_pt = self.ref_pt + move_to;
    }
}

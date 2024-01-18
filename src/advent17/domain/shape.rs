use crate::advent17::domain::point::Point;

use super::direction::Direction;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Shape {
    pub rel_pts: &'static [Point],
    pub ref_pt: Point,
    pub hight: i32,
}

impl Shape {
    pub const fn new(relative_pts: &'static [Point], hight: i32) -> Shape {
        Shape {
            rel_pts: relative_pts,
            ref_pt: Point::new(0, 0),
            hight,
        }
    }

    pub fn get_points(&self) -> Vec<Point> {
        self.rel_pts.iter().map(|pt| self.ref_pt + pt).collect()
    }

    pub fn get_max_hight(&self) -> i32 {
        self.hight + self.ref_pt.y
    }

    pub fn shift_to_pt(&mut self, move_to: &Point) {
        self.ref_pt = self.ref_pt + move_to;
    }

    pub fn shift(&mut self, move_to: Direction) {
        self.ref_pt = self.ref_pt + move_to.value();
    }

    pub fn colides(&self, other: &Shape) -> bool {
        for pt in self.get_points() {
            for other_pt in other.get_points() {
                if pt == other_pt {
                    return true;
                }
            }
        }
        return false;
    }
}

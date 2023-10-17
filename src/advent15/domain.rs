use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coord(pub i64, pub i64);

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Coord {
    fn norm(self, rhs: Coord) -> i64 {
        self.0 * rhs.1 - self.1 * rhs.0
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Area {
    pub A: (i64, i64),
    pub B: (i64, i64),
    pub C: (i64, i64),
    pub D: (i64, i64),
}

impl Area {
    pub fn new(sensor: Coord, mhd: i64) -> Area {
        let a = Coord(sensor.0, sensor.1 - mhd);
        let b = Coord(sensor.0 + mhd, sensor.1);
        let c = Coord(sensor.0, sensor.1 + mhd);
        let d = Coord(sensor.0 - mhd, sensor.1);
        Area {
            A: (1, a.1 - a.0),
            B: (-1, b.1 + b.0),
            C: (1, c.1 - c.0),
            D: (-1, d.1 + d.0),
        }
    }
}

pub fn distance(m: i64, c1: i64, c2: i64) -> f64 {
    f64::abs((c2 - c1) as f64) / f64::sqrt((m.pow(2) + 1) as f64)
}

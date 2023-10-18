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

pub fn potential_beacon(sensor: Coord, mhd: i64) -> Vec<Coord> {
    let a = Coord(sensor.0, sensor.1 - mhd - 1);
    let b = Coord(sensor.0 + mhd + 1, sensor.1);
    let c = Coord(sensor.0, sensor.1 + mhd + 1);
    let d = Coord(sensor.0 - mhd - 1, sensor.1);

    let mut potential_points = vec![];
    for (norm, source, target) in [
        (Coord(1, 1), a, b),
        (Coord(-1, 1), b, c),
        (Coord(-1, -1), c, d),
        (Coord(1, -1), d, a),
    ] {
        let mut new_point = source;
        while new_point != target {
            if new_point.0 >= 0
                && new_point.1 >= 0
                && new_point.0 <= 4000000
                && new_point.1 <= 4000000
            {
                potential_points.push(new_point);
            }
            new_point = new_point + norm;
        }
    }
    potential_points
}

pub fn distance(m: i64, c1: i64, c2: i64) -> f64 {
    f64::abs((c2 - c1) as f64) / f64::sqrt((m.pow(2) + 1) as f64)
}

mod domain;
mod parsing;

use crate::advent15::domain::*;
use crate::advent15::parsing::*;

use crate::advent15::parse_line;
use crate::helper;

fn manhattan_distance(source: Coord, target: Coord) -> u64 {
    i64::abs_diff(source.0, target.0) + i64::abs_diff(source.1, target.1)
}

fn is_not_occupied(sb: &Vec<(Coord, Coord, u64)>, other: Coord) -> bool {
    sb.iter().all(|x| x.0 != other && x.1 != other)
}

pub fn advent15() -> String {
    let content = helper::read_puzzle_input("./src/advent15/beacon.txt");
    let mut sensor_beacon = vec![];
    let mut min_x = 0;
    let mut max_x = 0;
    for line in content.lines() {
        let coords = parse_line(line);

        if coords.1 .0 > max_x {
            max_x = coords.1 .0;
        }
        if coords.1 .0 < min_x {
            min_x = coords.1 .0;
        }

        sensor_beacon.push((coords.0, coords.1, manhattan_distance(coords.0, coords.1)));
    }
    let mut negative_position_count = 0;
    'distress: for (_, source) in (-10i64.pow(7)..=10i64.pow(7)).enumerate() {
        // ranges are trail and error: )=
        let distress_beacon = Coord(source, 2000000);
        for (source, _, distance) in sensor_beacon.iter() {
            let mh = manhattan_distance(*source, distress_beacon);
            if mh <= *distance && is_not_occupied(&sensor_beacon, distress_beacon) {
                negative_position_count += 1;
                continue 'distress;
            }
        }
    }
    negative_position_count.to_string()
}

pub fn advent15_2() -> String {
    let content = helper::read_puzzle_input("./src/advent15/beacon.txt");
    let mut areas = vec![];
    for line in content.lines() {
        let (sensor, beacon) = parse_line(line);
        let mhd = manhattan_distance(sensor, beacon) as i64;
        areas.push(Area::new(sensor, mhd));
    }

    let mut A_dir = vec![];
    let mut B_dir = vec![];
    let mut C_dir = vec![];
    let mut D_dir = vec![];
    for area in areas {
        A_dir.push(area.A);
        B_dir.push(area.B);
        C_dir.push(area.C);
        D_dir.push(area.D);
    }

    let mut upwards = vec![];
    let mut downwards = vec![];

    for a in A_dir.iter() {
        for c in C_dir.iter() {
            let dist = distance(1, a.1, c.1);
            if dist - 0.707 < 0.01 {
                upwards.push((a, c));
                println!("found something {:?} {:?}", a, c)
            }
        }
    }
    println!();
    for b in B_dir.iter() {
        for d in D_dir.iter() {
            let dist = distance(-1, b.1, d.1);
            if dist - 0.707 < 0.01 {
                downwards.push((b, d));
                println!("found something {:?} {:?}", b, d)
            }
        }
    }

    let mut distress_signal = Coord(0, 0);

    dbg!(distress_signal);
    let tuning_frequency = 666; // distress_signal_pos.0 * 4000000 + distress_signal_pos.1;
    tuning_frequency.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_manhattan() {
        let source = Coord(8, 7);
        let target = Coord(2, 10);
        let res = manhattan_distance(source, target);
        assert_eq!(res, 9);
    }

    #[test]
    fn test_manhattan_negative() {
        let source = Coord(-8, 7);
        let target = Coord(2, 10);
        let res = manhattan_distance(source, target);
        assert_eq!(res, 13);
    }

    #[test]
    fn test_distance() {
        let c1 = Coord(1, 1);
        let c2 = Coord(1, 2);
        let dist = distance(Coord(1, 1), c1, c2);
        dbg!(dist);
        assert!(false);
    }

    #[test]
    fn test_cross() {
        let c1 = Coord(1, 1);
        let c2 = Coord(1, -1);
        let dist = c1.cross(c2);
        dbg!(dist);
        assert!(false);
    }
    #[test]
    fn test_area_distance() {
        let source = Coord(10, 10);
        let target = Coord(10, 11);
        let mhd = manhattan_distance(source, target);
        let area = Area::new(source, mhd as i64);
        let dist = distance(area.A.1, area.A.0, area.C.0);
        dbg!(dist);
        assert!(false);
    }
}

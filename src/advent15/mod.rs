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
    let mut points = vec![];
    let mut sensors = vec![];
    for (_count, line) in content.lines().enumerate() {
        let (sensor, beacon) = parse_line(line);
        let mhd = manhattan_distance(sensor, beacon) as i64;
        sensors.push((sensor, mhd));
        points.extend(potential_beacon(sensor, mhd));
    }
    let mut distress_signal = vec![];
    'distress: for potential_beacon in points {
        for (sensor, sensor_coverage) in sensors.iter() {
            let mhd = manhattan_distance(*sensor, potential_beacon);
            if mhd as i64 <= *sensor_coverage {
                continue 'distress;
            }
        }
        distress_signal.push(potential_beacon);
    }

    dbg!(&distress_signal);
    let tuning_frequency = distress_signal[0].0 * 4000000 + distress_signal[0].1;
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
}

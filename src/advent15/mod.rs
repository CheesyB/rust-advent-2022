mod domain;
mod parsing;

use crate::advent15::domain::*;
use crate::advent15::parsing::*;

use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

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

fn progress(thread_count: usize, loop_count: u64, max_x: u64, max_y: u64) {
    let percent = loop_count as f64 / (max_x * max_y) as f64 * 10 as f64;
    if (percent % 0.01 as f64) < 0.000001 as f64 && percent > 0.001 as f64 {
        println!("{} {:.1}%", thread_count, percent);
    }
}

fn check_range(
    count: usize,
    range: Vec<u64>,
    sensor_beacon: Vec<(Coord, Coord, u64)>,
) -> Option<Coord> {
    println!("{} checking range: {:?}", count, range);
    let max_y: u64 = 4000000;
    for (_, x) in (range[0]..range[1]).enumerate() {
        'distress_y: for (_, y) in (0..=max_y).enumerate() {
            progress(count, x + y, range[1] - range[0], max_y);
            let distress_beacon = Coord(x as i64, y as i64);
            for (sensor, _, source2beacon) in sensor_beacon.iter() {
                let source2distress = manhattan_distance(*sensor, distress_beacon);
                //println!("s2d: {},  s2b: {}", source2distress, source2beacon);
                if source2distress <= *source2beacon {
                    continue 'distress_y;
                }
            }
            return Some(distress_beacon);
        }
    }
    None
}

pub fn advent15_2() -> String {
    let content = helper::read_puzzle_input("./src/advent15/beacon.txt");
    let mut sensor_beacon = vec![];
    for line in content.lines() {
        let coords = parse_line(line);
        sensor_beacon.push((coords.0, coords.1, manhattan_distance(coords.0, coords.1)));
    }
    let max: u64 = 4000000;
    let num_of_threads = 8;
    let mut children = vec![];
    let intermediate = (0..=num_of_threads)
        .map(|x| x * max / num_of_threads)
        .collect::<Vec<u64>>();
    for (count, range) in intermediate.windows(2).enumerate() {
        let tmp_count = count.clone();
        let tmp_range = range.to_vec().clone();
        let tmp_sensor_beacon = sensor_beacon.clone();
        let tmp = thread::spawn(move || check_range(tmp_count, tmp_range, tmp_sensor_beacon));
        children.push(tmp);
    }
    let mut results = vec![];
    for j in children.into_iter() {
        results.push(j.join().unwrap());
    }

    let distress_signal_pos: Vec<Coord> = results.iter().filter_map(|r| *r).collect();
    dbg!(distress_signal_pos);
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
}

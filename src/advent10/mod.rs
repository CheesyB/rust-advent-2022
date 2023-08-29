use std::vec;

use crate::helper;

fn parse_line(input: &str) -> (&str, Option<i32>) {
    let mut split = input.split_whitespace();
    (
        split.next().unwrap(),
        split
            .next()
            .map(|val| i32::from_str_radix(val, 10).ok())
            .flatten(),
    )
}
#[derive(Debug, Copy, Clone)]
pub struct Mcu {
    cycle: i32,
    x: i32,
    signal: i32,
}

mod signal {
    use super::Mcu;

    static CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];

    pub fn tick(mcu: Mcu) -> Mcu {
        let mut new_mcu = mcu.clone();
        new_mcu.cycle += 1;
        signal(new_mcu)
    }
    pub fn signal(mcu: Mcu) -> Mcu {
        if CYCLES.contains(&mcu.cycle) {
            let mut new_mcu = mcu.clone();
            new_mcu.signal += new_mcu.x * new_mcu.cycle;
            return new_mcu;
        }
        return mcu;
    }

    pub fn add_x(mcu: Mcu, val: i32) -> Mcu {
        let mut new_mcu = mcu.clone();
        new_mcu.x += val;
        new_mcu
    }
}

mod screen {
    use super::Mcu;

    static CYCLES: [i32; 6] = [40, 80, 120, 160, 200, 240];

    fn crt(mcu: &Mcu, buffer: &mut Vec<&str>) {
        if CYCLES.contains(&(mcu.cycle - 1)) {
            buffer.push("\n");
        }
        let from = mcu.x - 1;
        let to = mcu.x + 1;
        if (from..=to).contains(&((mcu.cycle % 40) - 1)) {
            buffer.push("#");
        } else {
            buffer.push(".");
        }
    }

    pub fn tick(mcu: Mcu, buffer: &mut Vec<&str>) -> Mcu {
        let mut new_mcu = mcu.clone();
        new_mcu.cycle += 1;
        crt(&new_mcu, buffer);
        new_mcu
    }

    pub fn add_x(mcu: Mcu, val: i32) -> Mcu {
        let mut new_mcu = mcu.clone();
        new_mcu.x += val;
        new_mcu
    }
}

pub fn advent10_2() -> String {
    let content = helper::read_puzzle_input("./src/advent10/signal.txt");
    let mut buffer: Vec<&str> = vec!["\n"];
    let mut current = Mcu {
        cycle: 0,
        x: 1,
        signal: 0,
    };
    for line in content.lines() {
        let (instruction, arg) = parse_line(line);
        current = match instruction {
            "addx" => screen::add_x(
                screen::tick(screen::tick(current, &mut buffer), &mut buffer),
                arg.expect("should be there"),
            ),
            "noop" => screen::tick(current, &mut buffer),
            &_ => panic!("not good"),
        };
    }
    buffer.concat()
}

pub fn advent10() -> String {
    let content = helper::read_puzzle_input("./src/advent10/signal.txt");
    let mut current = Mcu {
        cycle: 0,
        x: 1,
        signal: 0,
    };
    for line in content.lines() {
        let (instruction, arg) = parse_line(line);
        current = match instruction {
            "addx" => signal::add_x(
                signal::tick(signal::tick(current)),
                arg.expect("should be there"),
            ),
            "noop" => signal::tick(current),
            &_ => panic!("not good"),
        };
    }

    current.signal.to_string()
}

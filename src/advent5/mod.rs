use std::fs;

fn basic() -> String {
    let file_path = "./src/advent5/crane.txt";
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

/*
            [G] [W]         [Q]
[Z]         [Q] [M]     [J] [F]
[V]         [V] [S] [F] [N] [R]
[T]         [F] [C] [H] [F] [W] [P]
[B] [L]     [L] [J] [C] [V] [D] [V]
[J] [V] [F] [N] [T] [T] [C] [Z] [W]
[G] [R] [Q] [H] [Q] [W] [Z] [G] [B]
[R] [J] [S] [Z] [R] [S] [D] [L] [J]
 1   2   3   4   5   6   7   8   9
*/

fn parse_commands(command_txt: &str) -> (usize, usize, usize) {
    let mut splits = command_txt.split_whitespace().skip(1).step_by(2);
    let (amount, from, to) = (
        usize::from_str_radix(splits.next().unwrap(), 10).unwrap(),
        usize::from_str_radix(splits.next().unwrap(), 10).unwrap() - 1,
        usize::from_str_radix(splits.next().unwrap(), 10).unwrap() - 1,
    );
    return (amount, from, to);
}

fn cargo_state() -> Vec<Vec<char>> {
    return vec![
        vec!['R', 'G', 'J', 'B', 'T', 'V', 'Z'],
        vec!['J', 'R', 'V', 'L'],
        vec!['S', 'Q', 'F'],
        vec!['Z', 'H', 'N', 'L', 'F', 'V', 'Q', 'G'],
        vec!['R', 'Q', 'T', 'J', 'C', 'S', 'M', 'W'],
        vec!['S', 'W', 'T', 'C', 'H', 'F'],
        vec!['D', 'Z', 'C', 'V', 'F', 'N', 'J'],
        vec!['L', 'G', 'Z', 'D', 'W', 'R', 'F', 'Q'],
        vec!['J', 'B', 'W', 'V', 'P'],
    ];
}

fn cargo_result(cargos: Vec<Vec<char>>) -> String {
    return cargos
        .iter()
        .map(|v: &Vec<char>| v.last().unwrap())
        .collect();
}

pub fn advent5_2() -> String {
    let moves = basic();

    let mut cargos = cargo_state();

    for line in moves.lines() {
        let command = parse_commands(line);
        let index_to_cut = cargos[command.1].len() - command.0;
        let mut slice = cargos[command.1].split_off(index_to_cut);
        cargos[command.2].append(&mut slice);
    }
    return cargo_result(cargos);
}

pub fn advent5() -> String {
    let moves = basic();
    let mut cargos = cargo_state();
    for line in moves.lines() {
        let command = parse_commands(line);
        for _ in 0..command.0 {
            let thing = cargos[command.1].pop().expect("wrong poping");
            cargos[command.2].push(thing);
        }
    }
    return cargo_result(cargos);
}

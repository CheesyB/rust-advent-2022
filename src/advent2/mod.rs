use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum RPS {
    A,
    B,
    C,
    X,
    Y,
    Z,
}
impl FromStr for RPS {
    type Err = ();

    fn from_str(input: &str) -> Result<RPS, Self::Err> {
        match input {
            "A" => Ok(RPS::A),
            "X" => Ok(RPS::X),
            "B" => Ok(RPS::B),
            "Y" => Ok(RPS::Y),
            "C" => Ok(RPS::C),
            "Z" => Ok(RPS::Z),
            _ => Err(()),
        }
    }
}

fn match_score3(round: (RPS, RPS)) -> Option<i32> {
    match round {
        (RPS::A, RPS::X) => Some(4),
        (RPS::A, RPS::Y) => Some(8),
        (RPS::A, RPS::Z) => Some(3),

        (RPS::B, RPS::X) => Some(1),
        (RPS::B, RPS::Y) => Some(5),
        (RPS::B, RPS::Z) => Some(9),

        (RPS::C, RPS::X) => Some(7),
        (RPS::C, RPS::Y) => Some(2),
        (RPS::C, RPS::Z) => Some(6),
        _ => None,
    }
}

/* X = LOOS=0, Y=DRAW=3, Z=WIN=6
    A=1=Rock, B=2=Paper, C=3=Scissors
*/
fn match_score4(round: (RPS, RPS)) -> Option<i32> {
    match round {
        (RPS::A, RPS::X) => Some(3),
        (RPS::A, RPS::Y) => Some(4),
        (RPS::A, RPS::Z) => Some(8),

        (RPS::B, RPS::X) => Some(1),
        (RPS::B, RPS::Y) => Some(5),
        (RPS::B, RPS::Z) => Some(9),

        (RPS::C, RPS::X) => Some(2),
        (RPS::C, RPS::Y) => Some(6),
        (RPS::C, RPS::Z) => Some(7),
        _ => None,
    }
}

pub fn advent2() -> String {
    let file_path = "./src/advent2/rock_paper_scissors.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut score = 0;
    for line in contents.lines() {
        let mut thing = line.split_whitespace();
        let enemy = RPS::from_str(thing.next().unwrap()).unwrap();
        let me = RPS::from_str(thing.next().unwrap()).unwrap();
        score += match_score3((enemy, me)).unwrap();
    }

    return score.to_string();
}

pub fn advent2_2() -> String {
    let file_path = "./src/advent2/rock_paper_scissors.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut score = 0;
    for line in contents.lines() {
        let mut thing = line.split_whitespace();
        let enemy = RPS::from_str(thing.next().unwrap()).unwrap();
        let me = RPS::from_str(thing.next().unwrap()).unwrap();
        score += match_score4((enemy, me)).unwrap();
    }

    return score.to_string();
}

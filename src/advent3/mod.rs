use std::fs;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn basic() -> String {
    let file_path = "./src/advent3/rucksack.txt";
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

pub fn advent3_2() -> i32 {
    let content = basic();
    let mut lines = content.lines();
    let mut score: usize = 0;
    loop {
        let (line1, line2, line3) = (lines.next(), lines.next(), lines.next());
        if line1.is_none() {
            break;
        };
        let present_in_all =
            find_duplicate_in_three((line1.unwrap(), line2.unwrap(), line3.unwrap()));
        score += calc_score(present_in_all);
    }
    return i32::try_from(score).unwrap();
}

pub fn advent3() -> i32 {
    let contents = basic();
    let mut score = 0;

    for line in contents.lines() {
        let halfs = line.split_at(line.len() / 2);
        let duplicate = find_duplicate(halfs);
        score += calc_score(duplicate);
    }

    return i32::try_from(score).unwrap();
}

fn calc_score(chr: char) -> usize {
    return ALPHABET.chars().position(|c| c == chr).unwrap() + 1;
}

fn find_duplicate_in_three(triple: (&str, &str, &str)) -> char {
    let mut res: char = 'a';
    triple.0.chars().for_each(|chr| {
        if triple.1.find(chr).is_some() {
            if triple.2.find(chr).is_some() {
                res = chr;
            }
        };
    });
    return res;
}

fn find_duplicate(halfs: (&str, &str)) -> char {
    let mut res: char = 'a';
    halfs.0.chars().for_each(|chr| {
        if halfs.1.find(chr).is_some() {
            res = chr;
        };
    });
    return res;
}

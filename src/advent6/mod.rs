use std::fs;

fn basic() -> String {
    let file_path = "./src/advent6/signal.txt";
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

fn is_maker(window: &[char]) -> bool {
    if window.len() == 0 {
        return true;
    }
    for chr in &window[1..] {
        if window[0] == *chr {
            return false;
        }
    }
    return is_maker(&window[1..]);
}

fn check_for_marker(signal: Vec<char>, marker_len: usize) -> usize {
    for i in marker_len..signal.len() {
        let window = &signal[i - marker_len..i];
        if is_maker(window) {
            return i;
        }
    }
    panic!("loop should run");
}

pub fn advent6() -> String {
    let signal: Vec<char> = basic().chars().collect();
    let index = check_for_marker(signal, 4);
    return index.to_string();
}

pub fn advent6_2() -> String {
    let signal: Vec<char> = basic().chars().collect();
    let index = check_for_marker(signal, 14);
    return index.to_string();
}

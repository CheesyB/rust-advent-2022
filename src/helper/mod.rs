use std::fs;

pub fn read_puzzle_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

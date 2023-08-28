use std::fs;

fn basic() -> String {
    let file_path = "./src/advent9/rope.txt";
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

pub fn advent9() -> String {
    return "Hallo".to_string();
}

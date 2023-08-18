use std::fs;

fn basic() -> String {
    let file_path = "./src/advent6/signal.txt";
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

pub fn advent6() -> String {
    return "hallo".to_owned();
}

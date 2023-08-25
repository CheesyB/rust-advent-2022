use fs;

fn basic() -> String {
    let file_path = "./src/advent7/source_tree.txt";
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

pub fn advent8() -> String {
    return "hallo".to_string();
}

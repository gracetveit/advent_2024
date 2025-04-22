use std::fs;

pub fn input_reader(filename: String) -> String {
    return fs::read_to_string(filename).expect("File read error!");
}

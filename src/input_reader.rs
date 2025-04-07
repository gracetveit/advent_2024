use std::fs;

pub fn input_reader(filename: String) -> String {
    let contents = fs::read_to_string(filename);
    match contents {
        Ok(read_contents) => {
            return read_contents;
        }
        Err(error) => {
            panic!("{error:?}");
        }
    }
}
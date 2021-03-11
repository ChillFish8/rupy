use rupy_parser::process_file;

use std::fs::read_to_string;

fn main() {
    let file = read_to_string("tests/test1.py").unwrap();

    process_file(&file);
}

use std::fs::read_to_string;

use rupy_parser::AstParser;

fn main() {
    let mut parse = AstParser::new();

    let file = read_to_string("tests/test1.py").unwrap();

    parse.parse_content(&file)
}

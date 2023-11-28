use std::{fs::File, io::Read};
mod parse;
fn main() {
    // Read input file
    let mut input_file = File::open("input.txt").expect("Failed to find 'input.txt'");
    let mut input_string = String::new();

    input_file.read_to_string(&mut input_string).expect("Failed to read file to string, invalid UTF-8 might be present");

    parse::parse(input_string);
}

use std::io::{stdin, Read};

mod types;

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
}

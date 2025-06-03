
use std::fs;

fn main() {
    fs::write("text.txt", "FROM RUST PROGRAM").unwrap();
}
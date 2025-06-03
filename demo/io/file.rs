
use std::fs;

fn main() {
    let text = fs::read_to_string("demo2.rs").unwrap();
    println!("{}", text);

    let t = fs::read("demo2.rs").unwrap();
    println!("{:?}", t);
}
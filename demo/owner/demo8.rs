fn main() {
    let mut s = String::from("hello");
    let str = &s;

    s.push_str(", world");

    println!("str: {}", str)
}
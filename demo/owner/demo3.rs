fn main() {
    let mut s = String::from("hello");
    let str = &mut s;
    str.push_str(", world!");

    println!("str: {}", str);
    println!("s: {}", s);
}
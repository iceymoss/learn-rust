fn main() {
    let s = String::from("hello");
    let len = get_len(s);

    println!("{} len: {}", s, len);
}

fn get_len(str: String) -> usize {
    str.len()
}
fn main() {
    let mut x: &str = define_x();
    println!("{}, world", x);
}

fn define_x() -> &'static str {
    let x1 = "hello";
    x1
}
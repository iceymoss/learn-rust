fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 不可变借用
    let r2 = &s; // 不可变借用
    let r3 = &mut s;

    println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
}
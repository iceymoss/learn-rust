fn main() {
    let mut s = String::from("hello");

    let r3 = &mut s;
    *r3 = String::from("iceymoss");
    println!("r3: {}", r3);

    let r1 = &s; // 不可变借用
    let r2 = &s; // 不可变借用

    println!("r1: {}, r2: {}", r1, r2);
}
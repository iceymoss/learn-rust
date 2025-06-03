fn main() {
    let a : i8 = 127;

    // 溢出处理
    let b = a.wrapping_add(3);
    println!("{}", b);  // 19

    let f1: f32 = 0.1;
    let f2: f32 = 0.2;
    println!("{}", f1 + f2);

    // 断言0.1 + 0.2与0.3相等
    assert!(0.1 + 0.2 == 0.3);
}
fn main() {
    let some_value = Some(5);

    // 传统 match
    match some_value {
        Some(x) => println!("值为: {}", x),
        None => (), // 什么都不做
    }

    // 使用 if let 更简洁
    if let Some(x) = some_value {
        println!("值为: {}", x);
    }
}
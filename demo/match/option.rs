fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    match plus_one(five) {
        None => println!("None"),
        Some(i) => println!("{}", i),
    }

    let none = plus_one(None);
    match none {
        None => println!("None"),
        Some(i) => println!("{}", i),
    }

    // Vec是动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // stack.pop从数组尾部弹出元素
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }


}

fn owner() {
    let s1 = String::from("hello");

    let len = owner_calculate_length(s1);

    //hello被传入了函数内容，函数内部没有发生移动，函数内部的所有者离开作用域，hello值被移除

    println!("The length of s1 is {}.", len);
}

fn reference() {
    let s1 = String::from("hello");

    //当一个变量的值被引用时，变量本身不会被认定无效。因为"引用"并没有在栈中复制变量的值
    //引用是指针，hello的所有者一直都是s1，未发送变化
    let len = reference_calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn rect() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}

fn owner_calculate_length(s: String) -> usize {
    s.len()
}

fn reference_calculate_length(s: &String) -> usize {
    s.len()
}


fn main() {
    owner();
    reference();
    rect();
}

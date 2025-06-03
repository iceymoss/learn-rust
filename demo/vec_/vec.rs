
fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3];  // 宏初始化
    vec.push(4);                  // 添加元素
    let popped: Option<i32> = vec.pop();       // 移除末尾元素
    println!("popped = {}", popped.unwrap());
    for (i, v) in vec.iter_mut().enumerate() {
        *v += 100;
        println!("vec[{}] = {}", i, v);
    }

    vec[1] = 100;
    println!("vec[1] = {}", vec[1]);
}
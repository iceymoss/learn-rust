fn main() {
    // -------------------------------
    // 2. 复合类型 (Compound Types)
    // -------------------------------

    // 元组 (Tuple) - 固定长度不同类型
    let tuple: (i32, f64, char) = (500, 6.28, 'θ');

    // 数组 (Array) - 固定长度相同类型
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    // -------------------------------
    // 3. 字符串类型
    // -------------------------------
    // &str：字符串切片（不可变视图）
    let greeting: &str = "Hello, Rustaceans!";

    // String：可增长的堆分配字符串
    let mut message = String::from("Learning");
    message.push_str(" Rust!");  // 修改字符串


    // 元组解构和访问
    let (x, y, z) = tuple;
    println!("元组解构: x={}, y={}, z={}", x, y, z);
    println!("元组索引: 索引1 = {}", tuple.1);

    println!("数组: {:?}", array);          // 使用 Debug trait 打印
    println!("数组首元素: {}", array[0]);    // 索引访问

    println!("字符串切片: {}", greeting);
    println!("可变字符串: {}", message);

    let mut x: i32 = 100;
    x = 101;

}
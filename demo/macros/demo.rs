
// 宏的定义
macro_rules! greet {
    // 模式匹配
    ($name:expr) => {
        // 宏的展开
        println!("Hello, {}!", $name);
    };
}

// 宏的定义
macro_rules! vec {
    // 基本情况，空的情况
    () => {
        Vec::new()
    };

    // 递归情况，带有元素的情况
    ($($element:expr),+ $(,)?) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($element);
            )+
            temp_vec
        }
    };
}


fn main() {
    // 调用宏
    greet!("World");
    // 调用宏
    let my_vec = vec![1, 2, 3];
    println!("{:?}", my_vec); // 输出: [1, 2, 3]

    let empty_vec = vec![];
    println!("{:?}", empty_vec); // 输出: []
}
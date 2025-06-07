fn main() {
    let s = String::from("hello");
    let str = &s; // 不可变借用：&s,借用范围开始

    println!("s: {}", s); // 打印所有权变量

    let str2 = &str; // 对str进行引用
    println!("str2: {}", str2);

    let str_temp = str; // 将str绑定非str_temp
    println!("str_temp: {}", str_temp);


    let str3 = &str2; // 对str2进行引用
    println!("str3: {}", str3);

    let str4 = &str3; // 对str3进行引用
    println!("str4: {}", str4);

    println!("str: {}", str);

    // 所有以下表达式访问相同数据
    // println!("{}", s);        // 原始变量
    // println!("{}", str);      // 一级引用
    // println!("{}", *str2);    // 二级引用解引用为一级
    // println!("{}", **str3);   // 三级引用解引用两次
    // println!("{}", ***str4);  // 四级引用解引用三次
}

fn calculate_length(str: &String) -> usize {
    // str 是一个指向 s 的引用
    str.len()
} // 引用离开作用域，但不会释放数据
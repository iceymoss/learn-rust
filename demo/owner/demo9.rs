fn main() {
    let r: &String;
    {
        let x = String::from("hello");
        r = &x; // x 离开作用域后，r 将成为悬垂引用
    } // x 在此被释放
    println!("r: {}", r); // 错误！使用悬垂引用
}
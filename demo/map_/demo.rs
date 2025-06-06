use std::collections::HashMap;

fn main() {
    let mut age = HashMap::new(); //声明一个哈希表，未指定类型, 在使用时自动推断类型
    age.insert("iceymoss", 18);
    println!("{:?}", age.get("iceymoss"));

    let mut year: HashMap<i32, String> = HashMap::new();
    year.insert(2000, "龙".to_string()); // 字符串切片转为String
    year.insert(2025, "蛇".to_string());
    println!("year: {:?}", year);

    let mut pass: HashMap<&str, bool> = HashMap::new();
    pass.insert("iceymoss", true);
    pass.insert("lass", false);
    pass.remove("lass");
    println!("pass: {:?}", pass);
    println!("len: {}", pass.len());
    println!("key: {:?}", pass.keys());
    println!("value: {:?}", pass.values());
}
use std::collections::HashMap;

// 结构体字段需标记为 pub 以允许外部访问（若结构体在模块外使用）
#[derive(Debug)] // 添加 Debug 以便打印
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Live {
    name: String,
    point: Point,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    likes: u8,
    gender: u8,
    live: Live,
}

fn main() {
    let mut user_info: HashMap<&str, Person> = HashMap::new();

    let p = Person {
        name: "iceymoss".to_string(),
        age: 20,
        likes: 5,
        gender: 0,
        live: Live {
            name: "ShangHai".to_string(),
            point: Point { x: 123.32, y: 38.2374 },
        },
    };

    // 插入时使用 &str 作为键
    user_info.insert("iceymoss", p);

    // 查询时同样使用 &str
    if let Some(person) = user_info.get("iceymoss") {
        println!("ming.name = {}", person.name);
        println!("ming.age = {}", person.age);
        println!("ming.likes = {}", person.likes);
        println!("ming.gender = {}", person.gender);
        println!("ming.live.name = {}", person.live.name);
        println!("ming.live.point.x = {}", person.live.point.x);
        println!("ming.live.point.y = {}", person.live.point.y);
    } else {
        println!("Person not found");
    }
}
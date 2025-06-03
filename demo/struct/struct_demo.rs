
struct Point {
    x: f32,
    y: f32,
}

struct Live {
    name: String,
    point: Point,
}

struct Person {
    name: String,
    age: u8,
    likes: u8,
    gender: u8,
    live: Live
}

fn main() {
    let ming: Person = Person {
        name: "iceymoss".to_string(),
        age: 20,
        likes: 5,
        gender: 0,
        live: Live {
            name: "ShangHai".to_string(),
            point: Point { x: 123.32, y: 38.2374 },
        }
    };

    println!("ming.name = {}", ming.name);
    println!("ming.age = {}", ming.age);
    println!("ming.likes = {}", ming.likes);
    println!("ming.gender = {}", ming.gender);
    println!("ming.live.name = {}", ming.live.name);
    println!("ming.points.x = {}", ming.live.point.x);
    println!("ming.points.y = {}", ming.live.point.y);

}
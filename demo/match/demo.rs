struct Rectangle {
    width: u32,
    height: u32
}

enum Shape {
    Rectangle(Rectangle),
    Triangle((u32, u32), (u32, u32), (u32, u32)),
    Circle { origin: (u32, u32), radius: u32 },
}

fn main() {
    let a_rec = Rectangle {
        width: 10,
        height: 20,
    };

    // 请打开下面这一行进行实验
    // let shape_a = Shape::Rectangle(a_rec);
    // 请打开下面这一行进行实验
    let shape_a = Shape::Triangle((0, 1), (3,4), (3, 0));

    let shape_a = Shape::Circle { origin: (0, 0), radius: 5 };

    // 这里演示了在模式匹配中将枚举的负载解出来的各种形式
    match shape_a {
        Shape::Rectangle(a_rec) => {  // 解出一个结构体
            println!("Rectangle {}, {}", a_rec.width, a_rec.height);
        }
        Shape::Triangle(x, y, z) => {  // 解出一个元组
            println!("Triangle {:?}, {:?}, {:?}", x, y, z);
        }
        Shape::Circle {origin, radius} => {  // 解出一个结构体的字段
            println!("Circle {:?}, {:?}", origin, radius);
        }
    }
}
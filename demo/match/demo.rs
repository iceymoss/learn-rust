struct Rectangle {
    width: u32,
    height: u32
}

enum Shape {
    Rectangle(Rectangle),
    Triangle((u32, u32), (u32, u32), (u32, u32)),
    Circle { origin: (u32, u32), radius: u32 },
}

fn shape_match(shape: Shape) {
    match shape {
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

fn main() {
    let rectangle = Shape::Rectangle(Rectangle {
        width: 10,
        height: 20,
    });

    let triangle = Shape::Triangle((0, 1), (3,4), (3, 0));

    let circle = Shape::Circle { origin: (0, 0), radius: 5 };

    shape_match(rectangle); // Rectangle 10, 20
    shape_match(triangle);  // Triangle (0, 1), (3, 4), (3, 0)
    shape_match(circle);    // Circle (0, 0), 5
}
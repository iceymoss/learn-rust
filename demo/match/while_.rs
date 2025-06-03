
#[derive(Debug)]
enum Shape {
    Rectangle,
    Triangle,
    Circle,
}

fn main() {
    let mut shape_a = Shape::Rectangle;
    let mut i = 0;
    while let Shape::Rectangle = shape_a {    // 注意这一句
        if i > 9 {
            println!("Greater than 9, quit!");
            shape_a = Shape::Circle;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            i += 1;
        }
    }
}

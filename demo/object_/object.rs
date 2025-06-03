
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }

    fn heighter(&self, rect: &Rectangle) -> bool {
        self.height > rect.height
    }

    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1's area is {}", rect1.area());
    println!("rect1's perimeter {}", rect1.perimeter());

    let rect2 = Rectangle { width: 100, height: 40 };
    println!("rect2 wider > rect2 wider: {}", rect2.wider(&rect1));
    println!("rect2 heigher > rect1 heigher {}", rect2.heighter(&rect1));

    let rect3 = Rectangle::create(10, 20);
    println!("rect3's is {:?}", rect3);
}
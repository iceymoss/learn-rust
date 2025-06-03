
// 需要注意的是，self 依然有所有权的概念：
//
// self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
// &self 表示该方法对 Rectangle 的不可变借用
// &mut self 表示可变借用
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 不可变借用
    fn width(&self) -> bool {
        self.width > 0
    }

    // 不可变借用
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 不可变借用
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 实例对象的所有权会移动给self
    fn get_width(self) -> u32 {
        self.width
    }

    // 实例对象的所有权会移动给self
    fn get_height(self) -> u32 {
        self.height
    }

    // 可变借用
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // 可变借用
    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };


    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    // rect2.get_height();
    // 所有权移动，rect2被释放
    // println!("area is {}", rect2.area());
    // 报错：^^^^^ value borrowed here after move

    let mut rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    rect3.set_width(30);
    rect3.set_height(50);

    println!("The rectangle has a nonzero width; it is {}", rect3.width);



}

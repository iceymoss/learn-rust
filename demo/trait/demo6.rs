trait Shape {
    fn play(&self) {    // 定义了play()方法
        println!("1");
    }
}
trait Circle : Shape {
    fn play(&self) {    // 也定义了play()方法
        println!("2");
    }
}
struct A;
impl Shape for A {}
impl Circle for A {}

impl A {
    fn play(&self) {    // 又直接在A上实现了play()方法
        println!("3");
    }
}

fn main() {
    let a = A;
    a.play();    // 调用类型A上实现的play()方法

    // 这种语法，叫做完全限定语法，是调用类型上某一个方法的完整路径表达。如果 impl 和 impl trait 时有同名方法，用这个语法就可以明确区分出来
    <A as Circle>::play(&a);  // 调用trait Circle上定义的play()方法
    <A as Shape>::play(&a);   // 调用trait Shape上定义的play()方法
}
//输出
// 3
// 2
// 1
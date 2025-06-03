trait Shape {
    fn area(&self) -> f64 {
        15.3
    }
}

// 语法说明：需要实现Circle，你必须实现Shape
// T: Circle
// 实际上表示：
// T: Circle + Shape
trait Circle : Shape {
    fn radius(&self) -> f64 {
        101.5
    }
}

struct A {}

impl Shape for A {
    // fn area(&self) -> f64 {
    //     100.0
    // }
}

impl Circle for A {
    fn radius(&self) -> f64 {
        100.1
    }
}

// struct B {}
// impl Circle for B { // 编译错误： the trait bound `B: Shape` is not satisfied
//     fn radius(&self) -> f64 {
//         100.0
//     }
// }

fn main() {
    let a = A {};
    println!("{}", a.area());
    println!("{}", a.radius());

    println!("{}", <A as Shape >::area(&a));
    println!("{}" ,<A as Circle >::radius(&a));

}
mod module_a {
    pub trait Shape {
        fn play(&self) {
            println!("1");
        }
    }

    pub struct A;
    impl Shape for A {}
}

pub mod module_b {
    use super::module_a::Shape;
    use super::module_a::A;  // 这里只引入了另一个模块中的类型

     pub fn doit() {
        let a = A;
        a.play();
    }
}

fn main() {
    module_b::doit();
}
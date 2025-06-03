trait TraitA {
    type Item;
}
struct Foo<T: TraitA<Item=String>> {  // 这里在约束表达式中对关联类型做了具化
    x: T
}

#[derive(Debug)]
struct A;
impl TraitA for A {
    type Item = String;
}

// struct B;
// impl TraitA for B {
//     type Item = f64;
// }

fn main() {
    let a = Foo {
        x: A,
    };


    println!("{:?}", a.x);
}
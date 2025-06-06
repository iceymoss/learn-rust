struct Person {
    name: String,
    age: i8
}

fn main() {
    let p = Person {
        name:String::from("wangcai"), //String在堆空间中
        age:3,
    };

    // 这里会发生所有权移动, 导致mydog.name被释放
    // let str = mydog.name;

    // 这里使用clone来处理，会创建mydog.name的一个副本
    // let str: String = p.name.clone();
    // println!("str={}", str);
    println!("person: name={},age={}", p.name, p.age);
}
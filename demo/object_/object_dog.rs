struct Dog {
    name: String,
    age: i8
}

fn main() {
    let mydog = Dog {
        name:String::from("wangcai"), //String在堆空间中
        age:3,
    };

    // 这里会发生所有权移动, 导致mydog.name被释放
    // let str = mydog.name;

    // 这里使用clone来处理，会创建mydog.name的一个副本
    let str: String = mydog.name.clone();
    println!("str={}", str);
    println!("mydog: name={},age={}", mydog.name, mydog.age);
}
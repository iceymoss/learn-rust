
fn main() {
    let s = String::from("hello");
    let s_temp = s.clone();
    let s1 = change(s); // 值的所有权移动到了s1, s已经被drop了
    println!("{}, {}", s_temp, s1);

    //引用
    let s2 = String::from("world");
    let s3 = change1(&s2);
    println!("{}, {}", s2, s3);

    //可变引用
    let mut s4 = String::from("hello");
    let s5 = change2(&mut s4);

    // 被引用期间是不可用的
    // println!("{}, {}", s4);

    println!("{}", *s5);
    println!("{}", s4);

    // 开启一片作用域, 对s4引用
    {
        let t = &mut s4;
        let _t = change2(t);
    }

    println!("{}", s4);


    let s6 = String::from("hello");
    let s7 = change3(s6);


    let mut s8  = String::from("hello");
    s8.push_str(&s7);
    println!("{}, {}", s7, s8);


    let s10 = String::from("hello");
    let s11 = String::from("world");
    let s12 = change4(s10, s11);
    println!("{}, {}", s12, s11);


}

fn change(str:String) -> String {
    str
}

fn change1(str: &String) -> &String {
    return str
}

fn change2(str: &mut String) -> &mut String {
    str.push_str(" world");
    str
}

fn change3(mut str: String) -> String {
    str.push_str(" world");
    str
}

fn change4(mut str1: String, str2: String) -> String {
    str1.push_str(str2.as_str());
    str1
}

fn main() {
    let str: &str = "hello world";
    let s1: &'static str = "I am a superman.";
    let s2: String = str.to_string();
    let s3: &String = &s2;
    let s4 = &s2[..];
    let s5 = &s2[1..2];
    println!("{}", s5);

    // str to String
    let s4: String = "hello world".to_string();
    let s5: &str = &s4[..];
    let s6: String = s5.to_string();
    println!("{}", s6);

    // slice
    let s4: String = String::from("hello world");
    let s5: &str = &s4[..];
    let s6: &str = &s4[..4];
    println!("{}", s6);

    // str to owner
    let s4: &str = "hello world";
    let s5: String = s4.to_string();
    let s6: String = String::from(s4);
    let s7: String = s4.to_owned();
    println!("{}", s7);

    // [u8] <=> vec<u8>
    let a_vec: Vec<u8> = vec![1,2,3,4,5,6,7,8];
    // a_slice 是 [1,2,3,4,5]
    let a_slice: &[u8] = &a_vec[0..5];
    // 用 .to_vec() 方法将切片转换成Vec
    let another_vec = a_slice.to_vec();
    // 或者用 .to_owned() 方法
    let another_vec = a_slice.to_owned();

    // as_str
    let s4: String = String::from("hello world");
    let s5: &str = s4.as_str();
    println!("{}", s5);

    let s6: &[u8] = s4.as_bytes();
    println!("{:?}", s6);

    // String的隐式转换
    let s4: String = String::from("hello world");
    foo(&s4);
    // let s5: &str = "hello world";
    // foo(s5);

    // string 被隐式转换为str了
    foo1(&s4);
    let s5: &str = "hello world";
    println!("{}", s5);
    // foo1(&s5);

    // 字符串切割成字符数组
    let s6: String = String::from("你好，吃了吗？");
    let s7: Vec<_> = s6.chars().collect();
    println!("{:?}", s7);

    // Parse 方法
    let a = "10".parse::<u32>();
    let aa: u32 = "10".parse().unwrap(); // 这种写法也很常见
    println!("{:?}", a);

    let a = "10".parse::<f32>();
    println!("{:?}", a);

    let a = "4.2".parse::<f32>();
    println!("{:?}", a);

    let a = "true".parse::<bool>();
    println!("{:?}", a);

    let a = "a".parse::<char>();
    println!("{:?}", a);

    let a = "192.168.1.100".parse::<std::net::IpAddr>();
    println!("{:?}", a);

}

fn foo(s: &String) {
    println!("{}", s);
}

fn foo1(s: &str) {
    println!("{}", s);
}
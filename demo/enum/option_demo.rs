
// Option是标准中
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }


    let opt1: Option<&str> = Option::None;
    match opt1 {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }

    // 由于Option是rust内部提供的，所以支持直接简化写法
    let t = Some(64);
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }

    //if let 语法
    let i = 1;
    match i {
        0 => println!("zero"),
        _ => {
            println!("unkown i");
        },
    }
}
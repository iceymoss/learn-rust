use std::io;
fn main() {
    // let name:String = String::from("iceymoss");
    let name = "iceymoss";

    println!("hello, {}", name);
    let e:(i32, f64, bool, char) = (100, 100.1, true, 'a');
    let (x, y, z, j) = e;
    println!("{}, {}, {}, {}", x, y, z, j);

    let array:[i32; 5] = [0, 1, 2, 3, 4];
    println!("{}", array[0]);

    println!("add:{}", add(10, 10));


    println!("请输入分数:");
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");

    let str_format = input.trim();
    match str_format.parse::<i8>() {
        Ok(score) => println!("{}", get_score(score)),
        Err(_) => println!("输入不合法"),
    }




}

fn add(x:i32, y:i32) -> i32 {
    return x + y
}

fn get_score(score:i8) -> String {
    let res = match score {
        0..=59 => {"不合格"},
        60..=79 => {"良好"},
        80..=100 => {"优秀"},
        _ => { "参数不合法"},
    };

    res.to_string()
}



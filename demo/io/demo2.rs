use std::io::stdin;

fn main() {
    let mut str_buf = String::new();

    // 从控制台接受参数，以 \n 结束
    stdin().read_line(&mut str_buf)
        .expect("Failed to read line.");

    println!("Your input line is \n{}", str_buf);
}
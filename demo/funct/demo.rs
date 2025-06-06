fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn print(str: &str) {
    println!("{}", str);
}

fn main() {
    let ans = add(10, 20);
    println!("add: {} + {} = {}", 10, 20, ans);
    print("hello, iceymoss");
}
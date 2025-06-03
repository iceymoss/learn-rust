
fn main() {
    let mut x: i32 = 0; //mut 表示可以变
    loop {
        x = x + 1;
        if x == 50 {
            break;
        }
        println!("{}", x);
    }
}
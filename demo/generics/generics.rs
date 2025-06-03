
// fn add<T>(a: T, b: T) -> T {
//     return a + b;
// }

fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

fn main() {
    println!("{}", add(10, 20));
    println!("{}", add(0.1, 0.22));
    println!("{}", add(100 as f64, 0.22));
}
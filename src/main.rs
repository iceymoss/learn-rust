// 声明自定义模块
mod fmt;
mod operations;
mod error;
mod file;

fn main() {
  fmt::print("Hello from custom fmt!").unwrap();
  let x: i32 = fmt::add(3, 2);
  println!("{}", x);
  
  let y: f32 = operations::add(1.0, 2.0);
  println!("{}", y);
  
  let y: f64 = operations::divide(20.0, 40.0).unwrap();
  
  println!("{}", y);
  
}

fn main() {
    let tuple: (i32, f32, bool) = (23, 32.4, true);
    // let (a, b, c) = tuple;
    let i: i32 = tuple.0;
    let f: f32 = tuple.1;
    let x: bool = tuple.2;
    println!("{}", i);
    println!("{}", f);
    println!("{}", x);
}
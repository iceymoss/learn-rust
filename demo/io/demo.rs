
fn main() {
    let args = std::env::args();
    println!("{:?}", args);
    for arg in args {
        println!("{}", arg);
    }
}
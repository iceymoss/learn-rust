use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for MyError {}

fn foo(num: u32) -> Result<String, Box<dyn Error>> {
    match num {
        10 => Ok("Hello world!".to_string()),
        _ => {
            let my_error = MyError;
            Err(Box::new(my_error))
        }
    }
}

fn main() {
    foo(1).expect("错误了");

}
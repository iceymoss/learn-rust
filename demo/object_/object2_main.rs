
mod object2;
use object2::ClassName;

fn main() {
    let object = ClassName::new(1024);
    object.public_method();
}
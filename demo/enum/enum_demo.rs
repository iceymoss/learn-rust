
#[derive(Debug)]
enum Book {
    Papery,
    Electronic
}

fn main() {
    let book = Book::Papery;
    println!("{:?}", book);
}

enum Book {
    Papery(u32),
    Electronic(String)
}

fn main() {
    let book = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else if let Book::Electronic(url) = &book { // 访问 Electronic 的字段
        println!("Electronic URL: {}", url);
    }
    else {
        println!("Not papery book");
    }
}
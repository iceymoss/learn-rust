enum Book {
    Papery {index: u32},
    Electronic {url: String},
}

fn main() {
    let book = Book::Papery{index: 1001};
    let ebook = Book::Electronic{url: String::from("http://mybook.com/electronic/bookname/index.html")};

    // match 枚举类实例 {
    //     分类1 => 返回值表达式,
    //     分类2 => 返回值表达式,
    //     ...
    // }
    match book {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        },
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }

    match ebook {
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
        Book::Papery { index } => {
            println!("Papery book {}", index);
        }
    }
}
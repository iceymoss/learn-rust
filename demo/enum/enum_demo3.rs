
// fn main() {
//     enum Book {
//         Papery(u32),
//         Electronic {url: String},
//     }
//     let book = Book::Papery(1001);
//
//     match book {
//         Book::Papery(i) => {
//             println!("{}", i);
//         },
//         Book::Electronic { url } => {
//             println!("{}", url);
//         }
//     }
// }

fn main() {
    let t = "abc";
    match t {
        "abc" => println!("Yes"),
        _ => {},
    }
}
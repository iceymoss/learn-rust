use std::fs::File;

// 可恢复异常在rust中是这样定义的：
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        },
        Err(err) => {
            println!("Failed to open the file: {:?}", err);
        }
    }

    let f1 = File::open("hello.go");
    match f1 {
        Ok(file) => {
            println!("File opened successfully.");
        }
        Err(err) => {
            println!("Failed to open the file: {:?}", err);
        }
    }

    // 简化流程
    let f3 = File::open("hello.txt");
    if let Ok(file) = f3 {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }


    // 如果想使一个可恢复错误按不可恢复错误处理，Result 类提供了两个办法：unwrap() 和 expect(message: &str) ：
    let f4 = File::open("hello.txt").unwrap();
    let f5 = File::open("hello.txt").expect("Failed to open.");
}
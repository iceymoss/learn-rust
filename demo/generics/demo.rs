fn max<T: PartialOrd>(array: &[T]) -> &T {
    let mut max_index = 0;
    for (i, item) in array.iter().enumerate() {
        if item > &array[max_index] {
            max_index = i;
        }
    }
    &array[max_index]
}

struct Point<T> {
    x: T,
    y: T,
}


// 使用示例
fn main() {
    let numbers = [34, 50, 25, 100, 65];
    println!("Max: {}", max(&numbers)); // 输出 Max: 100

    let integer = Point { x: 5, y: 10 };     // 一个整数point
    let float = Point { x: 1.0, y: 4.0 };    // 一个浮点数point
}
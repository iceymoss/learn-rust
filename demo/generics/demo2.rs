struct Point<T> {
    x: T,
    y: T
}

fn main() {
    let p1 = Point {
        x: 1.01,
        y: 1.32,
    };

    println!("{?}", p1.x);

}
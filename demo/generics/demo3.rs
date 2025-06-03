struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }

    fn location_name(&self) -> String {
        return "beijing".to_string();
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());
    println!("p.location_name = {}", p.location_name());
}
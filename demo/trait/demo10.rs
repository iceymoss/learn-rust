// 定义一个带类型参数的trait
trait Add<T> {
    type Output;
    fn add(self, rhs: T) -> Self::Output;
}

struct Point {
    x: i32,
    y: i32,
}

// 为 Point 实现 Add<Point> 这个 trait
impl Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// 为 Point 实现 Add<i32> 这个 trait
impl Add<i32> for Point {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = p1.add(p2);  // 两个Point实例相加
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);

    let p1 = Point { x: 1, y: 1 };
    let delta = 2;
    let p3 = p1.add(delta);   // 一个Point实例加一个i32
    assert_eq!(p3.x, 3);
    assert_eq!(p3.y, 3);
}
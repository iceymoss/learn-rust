use std::ops::Add;

// impl Add<i32> for i32 {
//     type Output = i32;
//     fn add(self, other: i32) -> Self::Output {
//         self - other
//     }
// }


// 定义新类型包装 i32
struct MyInt(i32);

// 为新类型实现 Add<i32>
impl Add<i32> for MyInt {
    type Output = i32;
    fn add(self, other: i32) -> Self::Output {
        self.0 - other // 实现减法逻辑
    }
}


fn main() {
    let my_num = MyInt(10);
    let result = my_num + 5; // 实际是 10 - 5 = 5
    println!("{}", result); // 输出 5
}
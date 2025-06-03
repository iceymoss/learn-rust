
fn main() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    //todo: 将切片的作用域限制在代码块内
    {
        let slice: &[i32] = &arr[1..4]; // 不可变借用
        for elem in slice {
            println!("{}", elem);       // 输出 2, 3, 4
        }
    } // 此处 slice 的借用结束

    // 现在可以安全修改 arr
    arr[3] = 10;
    println!("arr[3] = {}", arr[3]);    // 输出 arr[3] = 10

    // 重新创建切片以反映修改后的数据
    let new_slice: &[i32] = &arr[1..4];
    println!("new_slice[2] = {}", new_slice[2]); // 输出 new_slice[2] = 10

    println!("len: {}", arr.len());
}
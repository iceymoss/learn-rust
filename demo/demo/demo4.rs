
fn main() {
    let mut x: i32 = 100;
    while x != 0 {
        println!("value: {}", x);
        x -= 1;
    }

    let arr:[i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut index: i32 = 0;
    while index < arr.len() as i32 {
        println!("value: {}", arr[index as usize]);
        index += 1;
    }

}
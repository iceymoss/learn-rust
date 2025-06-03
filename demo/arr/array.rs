
fn main() {
    let mut arr: [i32; 5] = [-1, 2, 3, 4, 5];
    let mut total: i32 = 0;
    for i in arr.iter() {
        println!("{}", i);
        total += i;
    }

    let tag: i32 = arr[0];
    println!("tag: {}", tag);

    arr[4] = total;
    println!("total: {}", arr[4]);

}
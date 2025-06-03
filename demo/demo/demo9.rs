fn main() {
    let a: u8 = 255;
    let b = a>>7; // ok
    let b = a<<7; // ok
    // let b = a>>8; // overflow
    // let b = a<<8; // overflow


    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", size_of_val(&x));

    exp();
}

fn exp() {
    let y = {
        let x = 3;
        x + 1
    };


    println!("The value of y is: {}", y);
}
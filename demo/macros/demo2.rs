macro_rules! add {
    // 第一个分支，匹配两个元素的加法
    ($a:expr, $b:expr)=>{
        {
            $a+$b
        }
    };
    // 第二个分支：当只有一个元素时，也应该处理，这是边界情况
    ($a:expr)=>{
        {
            $a
        }
    }
}

fn main(){
    let x=0;
    let sum = add!(1,2);  // 调用宏
    println!("{}", sum);
    let sum = add!(x);
}
// rust-loop.rs
use std::time::Instant;

fn main() {
    const N: u64 = 1_000_000_000; // 10亿次迭代
    let start = Instant::now();

    let mut sum: u64 = 0;
    for i in 1..=N {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i * i;
        }
    }

    let duration = start.elapsed();
    println!("Rust 结果: {}, 耗时: {:?}", sum, duration);
}
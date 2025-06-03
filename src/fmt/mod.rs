//! 自定义格式化输出模块
//! 实现类似 Go 的 fmt.Print 功能

use std::io::{self, Write};

/// 模拟 Go 的 Print 函数
/// 自动添加换行符（与 Go 保持一致）
pub fn print(text: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(text.as_bytes())?;
    handle.write_all(b"\n")?;  // 自动换行
    handle.flush()
}

/// 扩展功能：支持格式化字符串
#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => {{
        $crate::fmt::print(&format!($($arg)*)).unwrap();
    }};
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
use crate::error::CalculatorError;

/// 加法运算
/// # 示例
/// ```
/// use calculator::operations::basic::add;
/// assert_eq!(add(2.5, 3.5), 6.0);
/// ```
pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

/// 安全除法
pub fn divide(a: f64, b: f64) -> Result<f64, CalculatorError> {
    if b == 0.0 {
        Err(CalculatorError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}
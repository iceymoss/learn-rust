use crate::error::CalculatorError;

/// 平方根计算
pub fn sqrt(x: f64) -> Result<f64, CalculatorError> {
    if x < 0.0 {
        Err(CalculatorError::NegativeRoot)
    } else {
        Ok(x.sqrt())
    }
}
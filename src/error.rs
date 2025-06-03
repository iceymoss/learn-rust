#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    DivisionByZero,
    NegativeRoot,
    InvalidInput,
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "Cannot divide by zero"),
            Self::NegativeRoot => write!(f, "Cannot calculate square root of negative number"),
            Self::InvalidInput => write!(f, "Invalid input detected"),
        }
    }
}

impl std::error::Error for CalculatorError {}
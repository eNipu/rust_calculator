// Define a custom error type
#[derive(Debug)]
pub enum CalcError {
    DivisionByZero,
    InvalidOperation,
    InvalidInput,
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalcError::DivisionByZero => write!(f, "division by zero"),
            CalcError::InvalidOperation => write!(f, "invalid operation"),
            CalcError::InvalidInput => write!(f, "input is NaN or infinity"),
        }
    }
}

// Implement std::error::Error for CalcError
impl std::error::Error for CalcError {}

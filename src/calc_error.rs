const ERROR_CODE_DIVISION_BY_ZERO: i32 = 500;
const ERROR_CODE_INVALID_INPUT: i32 = 501;

// Define a custom error type for the calculator
#[derive(Debug, PartialEq)]
pub enum CalcError {
    DivisionByZero,
    InvalidInput,
}

impl CalcError {
    pub fn code(&self) -> i32 {
        match self {
            CalcError::DivisionByZero => ERROR_CODE_DIVISION_BY_ZERO,
            CalcError::InvalidInput => ERROR_CODE_INVALID_INPUT,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            CalcError::DivisionByZero => "division by zero",
            CalcError::InvalidInput => "input is NaN or infinity",
        }
    }
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {}: {}", self.code(), self.message())
    }
}

impl std::error::Error for CalcError {}

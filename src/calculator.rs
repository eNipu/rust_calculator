use crate::calc_error::CalcError;

pub struct Calculator;

impl Calculator {
    pub fn new() -> Calculator {
        Calculator
    }

    pub fn add(&self, a: f64, b: f64) -> Result<f64, CalcError> {
        if a.is_nan() || a.is_infinite() || b.is_nan() || b.is_infinite() {
            Err(CalcError::InvalidInput)
        } else {
            Ok(a + b)
        }
    }

    pub fn subtract(&self, a: f64, b: f64) -> Result<f64, CalcError> {
        if a.is_nan() || a.is_infinite() || b.is_nan() || b.is_infinite() {
            Err(CalcError::InvalidInput)
        } else {
            Ok(a - b)
        }
    }

    pub fn multiply(&self, a: f64, b: f64) -> Result<f64, CalcError> {
        if a.is_nan() || a.is_infinite() || b.is_nan() || b.is_infinite() {
            Err(CalcError::InvalidInput)
        } else {
            Ok(a * b)
        }
    }

    pub fn divide(&self, a: f64, b: f64) -> Result<f64, CalcError> {
        if b == 0.0 {
            Err(CalcError::DivisionByZero)
        } else if a.is_nan() || a.is_infinite() || b.is_nan() || b.is_infinite() {
            Err(CalcError::InvalidInput)
        } else {
            Ok(a / b)
        }
    }
}

use crate::calc_error::CalcError;

pub struct Calculator;

impl Calculator {
    pub fn new() -> Calculator {
        Calculator
    }

    fn check_input(&self, a: f64, b: f64) -> Result<(), CalcError> {
        if a.is_nan() || a.is_infinite() || b.is_nan() || b.is_infinite() {
            Err(CalcError::InvalidInput)
        } else {
            Ok(())
        }
    }

    pub fn add(&self, a: f64, b: f64) -> Result<f64, CalcError> {
        self.check_input(a, b)?;
        Ok(a + b)
    }

    pub fn subtract(&self, a: f64, b: f64) -> Result<f64, CalcError> {
        self.check_input(a, b)?;
        Ok(a - b)
    }

    pub fn multiply(&self, a: f64, b: f64) -> Result<f64, CalcError> {
        self.check_input(a, b)?;
        Ok(a * b)
    }

    pub fn divide(&self, a: f64, b: f64) -> Result<f64, CalcError> {
        self.check_input(a, b)?;
        if b == 0.0 {
            Err(CalcError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
}

#[cfg(test)]
#[test]
fn test_add() {
    let calc = Calculator::new();
    match calc.add(2.0, 2.0) {
        Ok(result) => assert_eq!(result, 4.0),
        Err(_) => panic!("Unexpected error in add method"),
    }
}

#[test]
fn test_subtract() {
    let calc = Calculator::new();
    match calc.subtract(2.0, 2.0) {
        Ok(result) => assert_eq!(result, 0.0),
        Err(_) => panic!("Unexpected error in subtract method"),
    }
}

#[test]
fn test_multiply() {
    let calc = Calculator::new();
    match calc.multiply(2.0, 2.0) {
        Ok(result) => assert_eq!(result, 4.0),
        Err(_) => panic!("Unexpected error in multiply method"),
    }
}

#[test]
fn test_divide() {
    let calc = Calculator::new();
    match calc.divide(2.0, 2.0) {
        Ok(result) => assert_eq!(result, 1.0),
        Err(err) => panic!("Unexpected error in divide method: {}", err),
    }
}

#[test]
fn test_divide_by_zero() {
    let calc = Calculator::new();
    match calc.divide(2.0, 0.0) {
        Ok(_) => panic!("Expected error when dividing by zero"),
        Err(err) => assert_eq!(err, CalcError::DivisionByZero),
    }
}

#[test]
fn test_invalid_input() {
    let calc = Calculator::new();
    match calc.add(std::f64::NAN, 2.0) {
        Ok(_) => panic!("Expected error when adding NaN"),
        Err(err) => assert_eq!(err, CalcError::InvalidInput),
    }
}

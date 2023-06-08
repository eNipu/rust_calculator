mod calc_error;
mod calculator;

use pyo3::prelude::*;
use pyo3::types::PyDict;

// use crate::calc_error::CalcError;
use crate::calculator::Calculator;

// PyCalculator: Python binding for Calculator
#[pyclass]
struct PyCalculator {
    calc: Calculator,
}

// PyCalculatorError: Python binding for CalcError
#[pyclass]
pub struct PyCalculatorError {
    #[pyo3(get, set)]
    message: String,
    #[pyo3(get, set)]
    code: i32,
}

#[pymethods]
impl PyCalculatorError {
    #[new]
    fn new(message: String, code: i32) -> Self {
        PyCalculatorError { message, code }
    }
}


#[pymethods]
impl PyCalculator {
    #[new]
    #[args(kwargs = "**")]
    fn new(_kwargs: Option<&PyDict>) -> PyResult<PyCalculator> {
        Ok(PyCalculator {
            calc: Calculator::new(),
        })
    }

    fn add(&self, a: f64, b: f64) -> PyResult<f64> {
        match self.calc.add(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(PyErr::new::<PyCalculatorError, _>((err.to_string(), err.code()))),
        }
    }

    // similar changes for subtract, multiply, and divide
    fn subtract(&self, a: f64, b: f64) -> PyResult<f64> {
        match self.calc.subtract(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(PyErr::new::<PyCalculatorError, _>((err.to_string(), err.code()))),
        }
    }

    fn multiply(&self, a: f64, b: f64) -> PyResult<f64> {
        match self.calc.multiply(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(PyErr::new::<PyCalculatorError, _>((err.to_string(), err.code()))),
        }
    }

    fn divide(&self, a: f64, b: f64) -> PyResult<f64> {
        match self.calc.divide(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(PyErr::new::<PyCalculatorError, _>((err.to_string(), err.code()))),
        }
    }
}

// Python module
#[pymodule]
fn rust_calculator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyCalculator>()?;
    m.add_class::<PyCalculatorError>()?;
    Ok(())
}

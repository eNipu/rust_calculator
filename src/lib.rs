// We start by including the modules which contain the functionality of our library.
mod calc_error;
mod calculator;

// We import the Calculator and CalcError structs from our own modules, as well as some
// necessary functionality from the pyo3 library.
use crate::calc_error::CalcError;
use crate::calculator::Calculator;
use pyo3::create_exception;
use pyo3::prelude::*;

// PyCalculator: Python binding for Calculator
#[pyclass]
struct PyCalculator {
    calc: Calculator,
}

#[pymethods]
impl PyCalculator {
    #[new]
    fn new() -> PyResult<PyCalculator> {
        Ok(PyCalculator {
            calc: Calculator::new(),
        })
    }

    fn add(&self, a: f64, b: f64) -> PyResult<f64> {
        match self.calc.add(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(map_error(err)),
        }
    }

    fn subtract(&self, a: f64, b: f64) -> PyResult<f64> {
        match self.calc.subtract(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(map_error(err)),
        }
    }

    fn multiply(&self, a: f64, b: f64) -> PyResult<f64> {
        match self.calc.multiply(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(map_error(err)),
        }
    }

    fn divide(&self, a: f64, b: f64) -> PyResult<f64> {
        match self.calc.divide(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(map_error(err)),
        }
    }
}

#[pyclass]
pub struct PyCalculatorErrorPayload {
    message: String,
    code: i32,
}

#[pymethods]
impl PyCalculatorErrorPayload {
    #[new]
    fn new(message: String, code: i32) -> Self {
        PyCalculatorErrorPayload { message, code }
    }

    #[getter]
    fn get_message(&self) -> PyResult<String> {
        Ok(self.message.clone())
    }

    #[getter]
    fn get_code(&self) -> PyResult<i32> {
        Ok(self.code)
    }
}

// Create the Python exception class
create_exception!(
    rust_calculator,
    PyCalculatorError,
    pyo3::exceptions::PyException
);

fn map_error(err: CalcError) -> PyErr {
    let error_message = format!("{}", err);
    let error_payload = PyCalculatorErrorPayload::new(error_message, err.code());
    PyErr::new::<PyCalculatorError, _>(error_payload)
}

// Python module
#[pymodule]
fn rust_calculator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyCalculator>()?;
    m.add_class::<PyCalculatorErrorPayload>()?;
    Ok(())
}

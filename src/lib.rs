mod calc_error;
mod calculator;

use crate::calc_error::CalcError;
use crate::calculator::Calculator;
use pyo3::create_exception;
use pyo3::prelude::*;
use pyo3::types::PyDict;

// PyCalculator: Python binding for Calculator
#[pyclass]
struct PyCalculator {
    calc: Calculator,
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
        let gil = Python::acquire_gil();
        let py = gil.python();
        match self.calc.add(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(map_error(py, err)),
        }
    }

    fn subtract(&self, a: f64, b: f64) -> PyResult<f64> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        match self.calc.subtract(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(map_error(py, err)),
        }
    }

    fn multiply(&self, a: f64, b: f64) -> PyResult<f64> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        match self.calc.multiply(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(map_error(py, err)),
        }
    }

    fn divide(&self, a: f64, b: f64) -> PyResult<f64> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        match self.calc.divide(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(map_error(py, err)),
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

fn map_error(_py: Python, err: CalcError) -> PyErr {
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

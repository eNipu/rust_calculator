mod calc_error;
mod calculator;

use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;
use pyo3::exceptions::PyValueError;

use crate::calculator::Calculator;

// PyCalculator: Python binding for Calculator
#[pyclass]
struct PyCalculator {
    calc: Calculator,
}

#[pymethods]
impl PyCalculator {
    #[new]
    #[args(kwargs="**")]
    fn new(_kwargs: Option<&PyDict>) -> PyResult<PyCalculator> {
        Ok(PyCalculator {
            calc: Calculator::new(),
        })
    }

    fn add(&self, a: f64, b: f64) -> PyResult<f64> {
        match self.calc.add(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(PyValueError::new_err(format!("{}", err))),
        }
    }

    fn subtract(&self, a: f64, b: f64) -> PyResult<f64> {
        match self.calc.subtract(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(PyValueError::new_err(format!("{}", err))),
        }
    }

    fn multiply(&self, a: f64, b: f64) -> PyResult<f64> {
        match self.calc.multiply(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(PyValueError::new_err(format!("{}", err))),
        }
    }

    fn divide(&self, a: f64, b: f64) -> PyResult<f64> {
        match self.calc.divide(a, b) {
            Ok(result) => Ok(result),
            Err(err) => Err(PyValueError::new_err(format!("{}", err))),
        }
    }
}

// Python module
#[pymodule]
fn rust_calculator(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyCalculator>()?;
    Ok(())
}

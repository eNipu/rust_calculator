# Rust Calculator
![Rust](https://github.com/eNipu/rust_calculator/actions/workflows/rust.yml/badge.svg)

This is a simple calculator library implemented in Rust and exposed to Python using the PyO3 framework.

The purpose of this project is:
- To demonstrate how to create a Python module from Rust code using PyO3 and Maturin.
- To demonstrate how to propagate errors from Rust to Python.
- To demonstrate how to create custom error types in Rust.

## Structure

```
rust_calculator
├── Cargo.lock
├── Cargo.toml
├── pyproject.toml
├── src
│   ├── calc_error.rs
│   ├── calculator.rs
│   ├── lib.rs
│   └── main.rs
```

## Features
- Basic arithmetic operations: addition, subtraction, multiplication, and division.
- Error propagation from Rust to Python.
- Custom error types for division by zero and invalid operations.

## Requirements
- Rust 1.41 or later
- Python 3.7 or later
- pyO3
- maturin

## Usage

### Rust

You can test the Rust side using the provided `main.rs`:

```sh
cargo run
```

### Python

First, compile the Rust library into a Python module:

```sh
maturin develop
```

Then you can import and use the module in Python:

```python
import rust_calculator

calc = rust_calculator.PyCalculator()

# Perform some calculations
print(calc.add(2, 3))  # 5
print(calc.subtract(5, 3))  # 2
print(calc.multiply(2, 3.7))  # 7.4

# Attempt division by zero
try:
    print(calc.divide(2, 0))
except Exception as e:
    if type(e).__name__ == 'PyCalculatorError':
        print(f'Error Code: {e.args[0].code}, Message: {e.args[0].message}')
    else:
        print(e)
```

## Testing

### Rust

You can run the Rust unit tests with:

```sh
cargo test
```

### Python

You can also test the Python module (after compiling with Maturin) in your Python test suite as you would with any other Python module.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
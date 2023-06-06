mod calc_error;
mod calculator;
use crate::calculator::Calculator;

fn main() {
    let calc = Calculator::new();

    // Perform some calculations
    match calc.add(2.0, 3.0) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("Error: {}", err),
    }

    match calc.subtract(5.0, 3.0) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("Error: {}", err),
    }

    match calc.multiply(2.0, 3.0) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("Error: {}", err),
    }

    // Attempt division by zero
    match calc.divide(2.0, 0.0) {
        Ok(result) => println!("{}", result),
        Err(err) => println!("Error: {}", err),
    }
}

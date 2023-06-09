import rust_calculator
from calculator_exception import PyCalculatorException


calc = rust_calculator.PyCalculator()

try:
    # Perform some calculations
    print(calc.add(2, 3))  # 5
    print(calc.subtract(5, 3))  # 2
    print(calc.multiply(2, 3.7))  # 7.4

    # Attempt division by zero
    print(calc.divide(2, 0))

except Exception as e:
    print(PyCalculatorException(e))


try:
    calc.add(float('inf'), 2)
except Exception as e:
    print(PyCalculatorException(e))


try:
    calc.subtract(float('nan'), 2)
except Exception as e:
    print(PyCalculatorException(e))
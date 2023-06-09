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


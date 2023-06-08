import rust_calculator

calc = rust_calculator.PyCalculator()

# Perform some calculations
print(calc.add(2, 3))  # 5
print(calc.subtract(5, 3))  # 2
print(calc.multiply(2, 3.7))  # 6

# Attempt division by zero
try:
    print(calc.divide(2, 0))
except ValueError as e:
    print(e)  # "division by zero"


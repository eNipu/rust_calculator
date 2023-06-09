class PyCalculatorException(Exception):
    def __init__(self, err):
        if type(err).__name__ == 'PyCalculatorError':
            self.message = f'Error Code: {err.args[0].code}, Message: {err.args[0].message}'
        else:
            self.message = str(err)
        super().__init__(self.message)

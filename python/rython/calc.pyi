def sum_as_string(a: int, b: int) -> str: ...

class NewInt:
    def __init__(self, number: int) -> None:
        ''' Create NewInt object.

            Parameters:
                number (int): Integer number between -2_147_483_648 and 2_147_483_647
        '''
    def add(self, second_number: int) -> int:
        ''' Add second number to number.

            Parameters:
                second_number (int): Integer number between -2_147_483_648 and 2_147_483_647
            Returns:
                (int): Integer number after addition bounded between -2_147_483_648 and 2_147_483_647
        '''
    def mul(self, second_number: int) -> int:
        ''' Mul second number to number.

            Parameters:
                second_number (int): Integer number between -2_147_483_648 and 2_147_483_647
            Returns:
                (int): Integer number after mul bounded between -2_147_483_648 and 2_147_483_647
        '''
    def get_number(self) -> int:
        ''' Get number value.

            Returns:
                (int): Value of number
        '''

class NewFloat:
    def __init__(self, number: float) -> None:
        ''' Create NewFloat object.

            Parameters:
                number (float): Float number
        '''
    def add(self, second_number: float) -> float:
        ''' Add second number to number.

            Parameters:
                second_number (float): Float number
            Returns:
                (float): Float number after addition
        '''
    def mul(self, second_number: float) -> float:
        ''' Mul second number to number.

            Parameters:
                second_number (float): Float number
            Returns:
                (float): Float number after mul
        '''
    def get_number(self) -> float:
        ''' Get number value.

            Returns:
                (float): Value of number
        '''
